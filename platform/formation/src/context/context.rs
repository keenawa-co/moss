use derive_more::{Deref, DerefMut};
use moss_std::collection::{FxHashSet, VecDeque};
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

use super::{
    async_context::AsyncContext,
    entity::{Entity, EntityId, EntityMap, Slot},
    model::{Model, ModelContext},
    subscriber::{SubscriberSet, Subscription},
};

pub struct Reservation<T>(pub(crate) Slot<T>);

pub trait EventEmitter<E: Any>: 'static {}

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

pub struct ContextCell(pub(crate) RefCell<Context>);

impl ContextCell {
    pub fn borrow(&self) -> ContextRef {
        ContextRef(self.0.borrow())
    }

    pub fn borrow_mut(&self) -> ContextRefMut {
        ContextRefMut(self.0.borrow_mut())
    }
}

#[derive(Deref, DerefMut)]
pub struct ContextRef<'a>(Ref<'a, Context>);

#[derive(Deref, DerefMut)]
pub struct ContextRefMut<'a>(RefMut<'a, Context>);

type Handler = Box<dyn FnMut(&mut Context) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut Context) -> bool + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut Context) + 'static>;

pub struct Context {
    // pub(crate) this: std::sync::Weak<Self>,
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    pub(crate) pending_notifications: FxHashSet<EntityId>,
    pub(crate) pending_effects: VecDeque<Effect>,
    pub(crate) pending_updates: usize,
    pub(crate) entities: EntityMap,
    pub(crate) flushing_effects: bool,
    pub(crate) event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    pub(crate) release_listeners: SubscriberSet<EntityId, ReleaseListener>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

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
    ) -> R
    where
        T: 'static,
    {
        self.update(|ctx| {
            let mut entity = ctx.entities.lease(model);
            let result = update(&mut entity, &mut ModelContext::new(ctx, model.downgrade()));
            ctx.entities.end_lease(entity);
            result
        })
    }
}

impl Context {
    // pub fn to_async(&self) -> AsyncContext {
    //     AsyncContext {
    //         app: self.this.clone(),
    //     }
    // }

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
        E: Entity<T>,
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

    pub(crate) fn subscribe_internal<T, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(E, &Evt, &mut Context) -> bool + 'static,
    ) -> Subscription
    where
        T: 'static + EventEmitter<Evt>,
        E: Entity<T>,
        Evt: 'static,
    {
        let entity_id = entity.entity_id();
        let entity = entity.downgrade();

        self.new_subscription(
            entity_id,
            (
                TypeId::of::<Evt>(),
                Box::new(move |event, cx| {
                    let event: &Evt = event.downcast_ref().expect("invalid event type");
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
        E: Entity<W>,
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
        E: Entity<W>,
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
