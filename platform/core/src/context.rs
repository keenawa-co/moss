pub mod async_context;
pub mod entity;
pub mod model_context;
pub mod subscription;

mod utl;

use async_context::AsyncContext;
use entity::{AnyEntity, EntityId, EntityMap, Model, Slot};
use model_context::ModelContext;
use moss_std::collection::{FxHashMap, FxHashSet, VecDeque};
use std::{
    any::{type_name, Any, TypeId},
    borrow::Borrow,
    cell::RefCell,
    future::Future,
    rc::{Rc, Weak},
};
use subscription::{SubscriberSet, Subscription};

use crate::{
    executor::{BackgroundExecutor, MainThreadExecutor, Task},
    global::Global,
    platform::AnyPlatform,
};

pub struct Reservation<T>(pub(crate) Slot<T>);

pub trait AnyContext {
    type Result<T>;

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<Reservation<T>>;

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>;

    fn update_model<T, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R>
    where
        T: 'static;
}

pub trait EventEmitter<E: Any>: 'static {}

pub enum Effect {
    Notify {
        emitter: EntityId,
    },
    Emit {
        emitter: EntityId,
        event_type: TypeId,
        event: Box<dyn Any>,
    },
    Defer {
        callback: Box<dyn FnOnce(&mut Context) + 'static>,
    },
}

type Handler = Box<dyn FnMut(&mut Context) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut Context) -> bool + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut Context) + 'static>;

pub struct Context {
    this: Weak<RefCell<Self>>,
    background_executor: BackgroundExecutor,
    main_thread_executor: MainThreadExecutor,
    observers: SubscriberSet<EntityId, Handler>,
    pending_notifications: FxHashSet<EntityId>,
    pending_effects: VecDeque<Effect>,
    pending_updates: usize,
    entities: EntityMap,
    flushing_effects: bool,
    event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    release_listeners: SubscriberSet<EntityId, ReleaseListener>,
    globals_by_type: FxHashMap<TypeId, Box<dyn Any>>,
    global_observers: SubscriberSet<TypeId, Handler>,
}

impl AnyContext for Context {
    type Result<T> = T;

    fn reserve_model<T: 'static>(&mut self) -> Self::Result<Reservation<T>> {
        Reservation(self.entities.reserve())
    }

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|ctx| {
            let slot = ctx.entities.reserve();

            let entity = build_model(&mut ModelContext::new(ctx, slot.downgrade()));
            ctx.entities.insert(slot, entity)
        })
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Model<T> {
        self.update(|ctx| {
            let slot = reservation.0;
            let entity = build_model(&mut ModelContext::new(ctx, slot.downgrade()));
            ctx.entities.insert(slot, entity)
        })
    }

    fn update_model<T: 'static, R>(
        &mut self,
        model: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> R {
        self.update(|ctx| {
            let mut entity = ctx.entities.lease(model);
            let result = update(&mut entity, &mut ModelContext::new(ctx, model.downgrade()));
            ctx.entities.end_lease(entity);
            result
        })
    }
}

impl Context {
    pub fn new(platform: Rc<dyn AnyPlatform>) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|this| {
            RefCell::new(Context {
                this: this.clone(),
                background_executor: platform.background_executor().clone(),
                main_thread_executor: platform.main_thread_executor().clone(),
                observers: SubscriberSet::new(),
                pending_notifications: FxHashSet::default(),
                pending_effects: VecDeque::new(),
                pending_updates: 0,
                entities: EntityMap::new(),
                flushing_effects: false,
                event_listeners: SubscriberSet::new(),
                release_listeners: SubscriberSet::new(),
                globals_by_type: FxHashMap::default(),
                global_observers: SubscriberSet::new(),
            })
        })
    }

    pub fn global<G: Global>(&self) -> &G {
        self.globals_by_type
            .get(&TypeId::of::<G>())
            .map(|any_state| any_state.downcast_ref::<G>().unwrap())
            .ok_or_else(|| anyhow!("no state of type {} exists", type_name::<G>()))
            .unwrap()
    }

    pub fn set_global<G: Global>(&mut self, global: G) {
        let global_type = TypeId::of::<G>();

        // TODO: notify
        self.globals_by_type.insert(global_type, Box::new(global));
    }

    pub fn observe_global<G: Global>(
        &mut self,
        mut f: impl FnMut(&mut Self) + 'static,
    ) -> Subscription {
        let (subscription, activate) = self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |ctx| {
                f(ctx);
                true
            }),
        );
        self.defer(move |_| activate());
        subscription
    }

    pub fn spawn_local<Fut>(&self, f: impl FnOnce(AsyncContext) -> Fut) -> Task<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        self.main_thread_executor.spawn_local(f(self.to_async()))
    }

    pub fn block_on<R>(&self, fut: impl Future<Output = R>) -> R {
        self.background_executor.block_on(fut)
    }

    pub fn background_executor(&self) -> BackgroundExecutor {
        self.borrow().background_executor.clone()
    }

    pub fn to_async(&self) -> AsyncContext {
        AsyncContext {
            cell: self.this.clone(),
            background_executor: self.background_executor.clone(),
            main_thread_executor: self.main_thread_executor.clone(),
        }
    }

    pub fn defer(&mut self, f: impl FnOnce(&mut Context) + 'static) {
        self.push_effect(Effect::Defer {
            callback: Box::new(f),
        })
    }

    pub fn push_effect(&mut self, effect: Effect) {
        match &effect {
            Effect::Notify { emitter } => {
                if !self.pending_notifications.insert(*emitter) {
                    return;
                }
            }
            _ => {}
        };

        self.pending_effects.push_back(effect);
    }

    pub fn subscribe<T, E, Event>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Event, &mut Context) + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Event>,
        E: AnyEntity<T>,
        Event: 'static,
    {
        self.subscribe_internal(entity, move |entity, event, ctx| {
            on_event(entity, event, ctx);
            true
        })
    }

    pub(crate) fn new_subscription(
        &mut self,
        key: EntityId,
        value: (TypeId, Listener),
    ) -> Subscription {
        let (subscription, activate) = self.event_listeners.insert(key, value);
        self.defer(move |_| activate());
        subscription
    }

    pub(crate) fn subscribe_internal<T, E, Ev>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Ev, &mut Context) -> bool + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Ev>,
        E: AnyEntity<T>,
        Ev: 'static,
    {
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();

        self.new_subscription(
            entity_id,
            (
                TypeId::of::<Ev>(),
                Box::new(move |event, cx| {
                    let event: &Ev = event.downcast_ref().expect("invalid event type");
                    if let Some(handle) = E::upgrade_from(&entity) {
                        on_event(handle, event, cx)
                    } else {
                        false
                    }
                }),
            ),
        )
    }

    pub(crate) fn new_observer(&mut self, key: EntityId, value: Handler) -> Subscription {
        let (subscription, activate) = self.observers.insert(key, value);
        self.defer(move |_| activate());

        subscription
    }

    pub fn observe<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(E, &mut Context) + 'static,
    ) -> Subscription
    where
        W: 'static,
        E: AnyEntity<W>,
    {
        self.observe_internal(entity, move |e, ctx| {
            on_notify(e, ctx);
            true
        })
    }

    pub(crate) fn observe_internal<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(E, &mut Context) -> bool + 'static,
    ) -> Subscription
    where
        W: 'static,
        E: AnyEntity<W>,
    {
        let entity_id = entity.entity_id();
        let handle = entity.downgrade();
        self.new_observer(
            entity_id,
            Box::new(move |ctx| {
                if let Some(handle) = E::upgrade_from(&handle) {
                    on_notify(handle, ctx)
                } else {
                    false
                }
            }),
        )
    }

    pub(crate) fn update<R>(&mut self, update: impl FnOnce(&mut Context) -> R) -> R {
        self.pending_updates += 1;
        let result = update(self);
        if !self.flushing_effects && self.pending_updates == 1 {
            self.flushing_effects = true;
            self.flush_effects();
            self.flushing_effects = false;
        }

        self.pending_updates -= 1;
        result
    }

    fn flush_effects(&mut self) {
        loop {
            self.release_dropped_entities();

            if let Some(effect) = self.pending_effects.pop_front() {
                match effect {
                    Effect::Notify { emitter } => {
                        self.apply_notify_effect(emitter);
                    }
                    Effect::Emit {
                        emitter,
                        event_type,
                        event,
                    } => self.apply_emit_effect(emitter, event_type, event),
                    Effect::Defer { callback } => {
                        self.apply_defer_effect(callback);
                    }
                }
            } else {
                if self.pending_effects.is_empty() {
                    break;
                }
            }
        }
    }

    fn release_dropped_entities(&mut self) {
        loop {
            let dropped = self.entities.take_dropped();
            if dropped.is_empty() {
                break;
            }

            for (entity_id, mut entity) in dropped {
                self.observers.remove(&entity_id);
                self.event_listeners.remove(&entity_id);
                for release_callback in self.release_listeners.remove(&entity_id) {
                    release_callback(entity.as_mut(), self);
                }
            }
        }
    }

    fn apply_notify_effect(&mut self, emitter: EntityId) {
        self.pending_notifications.remove(&emitter);

        self.observers
            .clone()
            .retain(&emitter, |handler| handler(self));
    }

    fn apply_defer_effect(&mut self, callback: Box<dyn FnOnce(&mut Context) + 'static>) {
        callback(self);
    }

    fn apply_emit_effect(&mut self, emitter: EntityId, event_type: TypeId, event: Box<dyn Any>) {
        self.event_listeners
            .clone()
            .retain(&emitter, |(stored_type, handler)| {
                if *stored_type == event_type {
                    handler(event.as_ref(), self)
                } else {
                    true
                }
            });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(Debug)]
//     struct Counter {
//         count: i32,
//     }

//     #[derive(Debug)]
//     struct UI {
//         display_text: String,
//     }

//     #[derive(Debug)]
//     struct Change {
//         delta: i32,
//     }

//     impl EventEmitter<Change> for Counter {}

//     #[test]
//     fn test_subscription_via_global_context() {
//         struct UserStatus {
//             online: bool,
//         }

//         let mut ctx = PlatformContext::new();

//         let user_status = ctx.new_model(|_cx| UserStatus { online: false });

//         let s = ctx.observe(&user_status, |status_model, _cx| {
//             let status = status_model.read(_cx);
//             if status.online {
//                 dbg!("User is now online.");
//             } else {
//                 dbg!("User is now offline.");
//             }
//         });

//         user_status.update(&mut ctx, |status, cx| {
//             status.online = true;
//             cx.notify();
//         });

//         user_status.update(&mut ctx, |status, cx| {
//             status.online = false;
//             cx.notify();
//         });
//     }

//     #[test]
//     fn test_counter_ui_integration() {
//         let mut ctx = PlatformContext::new();
//         let counter: Model<Counter> = ctx.new_model(|_cx| Counter { count: 0 });

//         let ui: Model<UI> = ctx.new_model(|cx: &mut ModelContext<UI>| {
//             cx.subscribe(&counter, |ui, _counter, event: &Change, _cx| {
//                 ui.display_text = format!("Counter: {}", event.delta);
//             })
//             .detach();

//             UI {
//                 display_text: format!("Counter: 0"),
//             }
//         });

//         counter.update(&mut ctx, |counter, cx| {
//             counter.count += 5;
//             cx.notify();
//             cx.emit(Change {
//                 delta: counter.count,
//             });
//         });

//         assert_eq!(ui.read(&mut ctx).display_text, "Counter: 5");
//     }
// }
