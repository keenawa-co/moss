use moss_std::collection::{BTreeMap, BTreeSet};
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::any::TypeId;
use std::fmt::Debug;
use std::mem;
use std::ops::AddAssign;
use std::sync::Weak;
use std::{any::Any, cell::Cell, collections::VecDeque, rc::Rc, sync::Arc};

// slotmap::new_key_type! {
//     pub struct EntityId;
// }

// pub enum Effect {
//     Notify { emitter: EntityId },
// }

// pub struct Entity {
//     id: EntityId,
//     context: Weak<Context>,
// }

// impl Entity {
//     pub fn new(id: EntityId, context: Weak<Context>) -> Self {
//         Self { id, context }
//     }

//     pub fn notify(&self) {
//         if let Some(mut context) = self.context.upgrade() {
//             context.add_effect(Effect::Notify { emitter: self.id });
//         }
//     }
// }
// pub struct EntityMap {
//     entities: SlotMap<EntityId, Box<dyn Any>>,
// }

// impl EntityMap {
//     pub fn new() -> Self {
//         Self {
//             entities: SlotMap::with_key(),
//         }
//     }

//     fn insert(&mut self, entity: Box<dyn Any>) -> EntityId {
//         self.entities.insert(entity)
//     }
// }

// pub struct Subscriber<Callback> {
//     active: Rc<Cell<bool>>,
//     callback: Callback,
// }

// pub struct SubscriberSet<EmitterKey, Callback>(
//     Arc<Mutex<SubscriberSetState<EmitterKey, Callback>>>,
// );

// pub struct SubscriberSetState<EmitterKey, Callback> {
//     subscribers: BTreeMap<EmitterKey, Option<BTreeMap<usize, Subscriber<Callback>>>>,
//     dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
//     next_subscriber_id: usize,
// }

// pub struct Subscription {
//     unsubscribe: Option<Box<dyn FnOnce() + 'static>>,
// }

// impl Subscription {
//     pub fn new(unsubscribe: impl 'static + FnOnce()) -> Self {
//         Self {
//             unsubscribe: Some(Box::new(unsubscribe)),
//         }
//     }

//     pub fn detach(mut self) {
//         self.unsubscribe.take();
//     }
// }

// impl Drop for Subscription {
//     fn drop(&mut self) {
//         if let Some(unsubscribe) = self.unsubscribe.take() {
//             unsubscribe();
//         }
//     }
// }

pub fn post_inc<T: From<u8> + AddAssign<T> + Copy>(value: &mut T) -> T {
    let prev = *value;
    *value += T::from(1);
    prev
}

// impl<EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
// where
//     EmitterKey: 'static + Ord + Clone + Debug,
//     Callback: 'static,
// {
//     fn new() -> Self {
//         Self(Arc::new(Mutex::new(SubscriberSetState {
//             subscribers: Default::default(),
//             dropped_subscribers: Default::default(),
//             next_subscriber_id: 0,
//         })))
//     }

//     fn insert(&self, emitter_key: EmitterKey, callback: Callback) -> (Subscription, impl FnOnce()) {
//         let active = Rc::new(Cell::new(false));
//         let mut lock = self.0.lock();
//         let subscriber_id = post_inc(&mut lock.next_subscriber_id);

//         lock.subscribers
//             .entry(emitter_key.clone())
//             .or_default()
//             .get_or_insert_with(Default::default)
//             .insert(
//                 subscriber_id,
//                 Subscriber {
//                     active: active.clone(),
//                     callback,
//                 },
//             );

//         let this = self.0.clone();

//         let subscription = Subscription {
//             unsubscribe: Some(Box::new(move || {
//                 let mut lock = this.lock();
//                 let Some(subscribers) = lock.subscribers.get_mut(&emitter_key) else {
//                     return;
//                 };

//                 if let Some(subscribers) = subscribers {
//                     subscribers.remove(&subscriber_id);
//                     if subscribers.is_empty() {
//                         lock.subscribers.remove(&emitter_key);
//                     }

//                     return;
//                 }

//                 lock.dropped_subscribers
//                     .insert((emitter_key, subscriber_id));
//             })),
//         };

//         (subscription, move || active.set(true))
//     }
// }

// pub struct Context {
//     entities: EntityMap,
//     pending_effects: VecDeque<Effect>,
//     observers: SubscriberSet<EntityId, Box<dyn FnMut(&mut Context)>>,
// }

// impl Context {
//     fn new() -> Self {
//         Self {
//             entities: EntityMap::new(),
//             pending_effects: VecDeque::new(),
//             observers: SubscriberSet::new(),
//         }
//     }

//     pub fn add_effect(&mut self, effect: Effect) {
//         self.pending_effects.push_back(effect);
//     }

//     pub fn register_entity(&mut self, entity: Box<dyn Any>) -> EntityId {
//         self.entities.insert(entity)
//     }

//     pub fn subscribe_notify(
//         &mut self,
//         entity_id: EntityId,
//         callback: Box<dyn FnMut(&mut Context)>,
//     ) -> Subscription {
//         let (subscription, activate) = self.observers.insert(entity_id, callback);
//         activate();
//         subscription
//     }
// }

slotmap::new_key_type! {
    pub struct EntityId;
}

// Перечисление для эффектов
#[derive(Debug)]
enum Effect {
    Notify {
        emitter: EntityId,
    },
    Emit {
        emitter: EntityId,
        event_type: TypeId,
        event: Box<dyn Any>,
    },
    // Callback(Box<dyn FnOnce(&mut Context)>),
}

// Основной контекст приложения
pub struct Context {
    pending_effects: VecDeque<Effect>,
    observers: SubscriberSet<EntityId, Box<dyn FnMut(&mut Context)>>,
    event_listeners: SubscriberSet<(EntityId, TypeId), Box<dyn Fn(&dyn Any, &mut Context)>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            pending_effects: VecDeque::new(),
            observers: SubscriberSet::new(),
            event_listeners: SubscriberSet::new(),
        }
    }

    // Добавление эффекта в очередь
    pub fn add_effect(&mut self, effect: Effect) {
        self.pending_effects.push_back(effect);
    }

    // Метод для обработки всех накопленных эффектов
    pub fn flush_effects(&mut self) {
        while let Some(effect) = self.pending_effects.pop_front() {
            match effect {
                Effect::Notify { emitter } => {
                    self.observers.retain(&emitter, |callback| {
                        callback(self);
                        true
                    });
                }
                Effect::Emit {
                    emitter,
                    event_type,
                    event,
                } => {
                    self.event_listeners
                        .retain(&(emitter, event_type), |callback| {
                            callback(event.as_ref(), self);
                            true
                        });
                } // Effect::Callback(callback) => {
                  //     callback(self);
                  // }
            }
        }
    }

    // Подписка на уведомления
    pub fn subscribe_notify(
        &mut self,
        entity_id: EntityId,
        callback: Box<dyn FnMut(&mut Context)>,
    ) -> Subscription {
        let (subscription, activate) = self.observers.insert(entity_id, callback);
        activate();
        subscription
    }

    // Подписка на события
    pub fn subscribe_emit(
        &mut self,
        entity_id: EntityId,
        event_type: TypeId,
        callback: Box<dyn Fn(&dyn Any, &mut Context)>,
    ) -> Subscription {
        let (subscription, activate) = self
            .event_listeners
            .insert((entity_id, event_type), callback);
        activate();
        subscription
    }

    // Метод для добавления отложенного колбека
    // pub fn add_callback(&mut self, callback: Box<dyn FnOnce(&mut Context)>) {
    //     self.add_effect(Effect::Callback(callback));
    // }
}

// Сущность, которая может генерировать уведомления и события
pub struct Entity {
    id: EntityId,
    context: Weak<Context>,
}

impl Entity {
    pub fn new(id: EntityId, context: Weak<Context>) -> Self {
        Self { id, context }
    }

    // Метод для создания уведомления
    pub fn notify(&self) {
        if let Some(mut context) = self.context.upgrade() {
            context.add_effect(Effect::Notify { emitter: self.id });
        }
    }

    // Метод для создания события
    pub fn emit<T: 'static + Any>(&self, event: T) {
        if let Some(mut context) = self.context.upgrade() {
            context.add_effect(Effect::Emit {
                emitter: self.id,
                event_type: TypeId::of::<T>(),
                event: Box::new(event),
            });
        }
    }

    // Метод для добавления отложенного колбека
    // pub fn defer(&self, callback: Box<dyn FnOnce(&mut Context)>) {
    //     if let Some(context) = self.context.upgrade() {
    //         context.add_callback(callback);
    //     }
    // }
}

// SubscriberSet и Subscription

pub(crate) struct SubscriberSet<EmitterKey, Callback>(
    Arc<Mutex<SubscriberSetState<EmitterKey, Callback>>>,
);

impl<EmitterKey, Callback> Clone for SubscriberSet<EmitterKey, Callback> {
    fn clone(&self) -> Self {
        SubscriberSet(self.0.clone())
    }
}

struct SubscriberSetState<EmitterKey, Callback> {
    subscribers: BTreeMap<EmitterKey, Option<BTreeMap<usize, Subscriber<Callback>>>>,
    dropped_subscribers: BTreeSet<(EmitterKey, usize)>,
    next_subscriber_id: usize,
}

struct Subscriber<Callback> {
    active: Rc<Cell<bool>>,
    callback: Callback,
}

impl<EmitterKey, Callback> SubscriberSet<EmitterKey, Callback>
where
    EmitterKey: 'static + Ord + Clone + Debug,
    Callback: 'static,
{
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(SubscriberSetState {
            subscribers: Default::default(),
            dropped_subscribers: Default::default(),
            next_subscriber_id: 0,
        })))
    }

    pub fn insert(
        &self,
        emitter_key: EmitterKey,
        callback: Callback,
    ) -> (Subscription, impl FnOnce()) {
        let active = Rc::new(Cell::new(false));
        let mut lock = self.0.lock();
        let subscriber_id = post_inc(&mut lock.next_subscriber_id);
        lock.subscribers
            .entry(emitter_key.clone())
            .or_default()
            .get_or_insert_with(Default::default)
            .insert(
                subscriber_id,
                Subscriber {
                    active: active.clone(),
                    callback,
                },
            );
        let this = self.0.clone();

        let subscription = Subscription {
            unsubscribe: Some(Box::new(move || {
                let mut lock = this.lock();
                let Some(subscribers) = lock.subscribers.get_mut(&emitter_key) else {
                    return;
                };

                if let Some(subscribers) = subscribers {
                    subscribers.remove(&subscriber_id);
                    if subscribers.is_empty() {
                        lock.subscribers.remove(&emitter_key);
                    }
                    return;
                }

                lock.dropped_subscribers
                    .insert((emitter_key, subscriber_id));
            })),
        };
        (subscription, move || active.set(true))
    }

    pub fn remove(&self, emitter: &EmitterKey) -> impl IntoIterator<Item = Callback> {
        let subscribers = self.0.lock().subscribers.remove(emitter);
        subscribers
            .unwrap_or_default()
            .map(|s| s.into_values())
            .into_iter()
            .flatten()
            .filter_map(|subscriber| {
                if subscriber.active.get() {
                    Some(subscriber.callback)
                } else {
                    None
                }
            })
    }

    pub fn retain<F>(&self, emitter: &EmitterKey, mut f: F)
    where
        F: FnMut(&mut Callback) -> bool,
    {
        let Some(mut subscribers) = self
            .0
            .lock()
            .subscribers
            .get_mut(emitter)
            .and_then(|s| s.take())
        else {
            return;
        };

        subscribers.retain(|_, subscriber| {
            if subscriber.active.get() {
                f(&mut subscriber.callback)
            } else {
                true
            }
        });
        let mut lock = self.0.lock();

        if let Some(Some(new_subscribers)) = lock.subscribers.remove(emitter) {
            subscribers.extend(new_subscribers);
        }

        for (dropped_emitter, dropped_subscription_id) in mem::take(&mut lock.dropped_subscribers) {
            debug_assert_eq!(*emitter, dropped_emitter);
            subscribers.remove(&dropped_subscription_id);
        }

        if !subscribers.is_empty() {
            lock.subscribers.insert(emitter.clone(), Some(subscribers));
        }
    }
}

// Subscription

pub struct Subscription {
    unsubscribe: Option<Box<dyn FnOnce() + 'static>>,
}

impl Subscription {
    pub fn new(unsubscribe: impl 'static + FnOnce()) -> Self {
        Self {
            unsubscribe: Some(Box::new(unsubscribe)),
        }
    }

    pub fn detach(mut self) {
        self.unsubscribe.take();
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(unsubscribe) = self.unsubscribe.take() {
            unsubscribe();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut context = Context::new();
        let entity_id = EntityId::from(1);
    }
}
