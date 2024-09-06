pub mod entity;
pub mod model_context;
pub mod subscription;

mod async_runner;

use async_runner::{Executor, Task};
use derive_more::{Deref, DerefMut};
use entity::{EntityId, EntityMap};
use hashbrown::HashMap;
use moss_std::collection::{FxHashSet, VecDeque};
use parking_lot::Mutex;
use std::{
    any::{Any, TypeId},
    cell::{RefCell, UnsafeCell},
    future::Future,
    rc::{Rc, Weak},
};
use subscription::{SubscriberSet, Subscription};

pub struct Context {
    inner: Rc<RefCell<ContextInner>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            inner: Rc::new_cyclic(|this| {
                RefCell::new(ContextInner {
                    this: this.clone(),
                    observers: SubscriberSet::new(),
                    pending_notifications: FxHashSet::default(),
                    pending_effects: VecDeque::new(),
                    pending_updates: 0,
                    entities: HashMap::new(),
                    flushing_effects: false,
                    event_listeners: SubscriberSet::new(),
                    release_listeners: SubscriberSet::new(),
                })
            }),
        }
    }

    // pub(crate) fn new_observer(&mut self, key: EntityId, value: Handler) -> Subscription {
    //     let (subscription, activate) = self.observers.insert(key, value);
    //     self.defer(move |_| activate());

    //     subscription
    // }

    // pub fn defer(&mut self, f: impl FnOnce(&mut Context) + 'static) {
    //     self.push_effect(Effect::Defer {
    //         callback: Box::new(f),
    //     })
    // }
}

impl Context {}

pub enum Effect {
    Notify {
        emitter: EntityId,
    },
    // Emit {
    //     emitter: EntityId,
    //     event_type: TypeId,
    //     event: Box<dyn Any>,
    // },
    Defer {
        callback: Box<dyn FnOnce(&mut Context) + 'static>,
    },
}

type Handler = Box<dyn FnMut(&mut Context) -> bool + 'static>;
type Listener = Box<dyn FnMut(&dyn Any, &mut Context) -> bool + 'static>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut Context) + 'static>;

pub struct ContextInner {
    this: Weak<RefCell<ContextInner>>,
    observers: SubscriberSet<EntityId, Handler>,
    pending_notifications: FxHashSet<EntityId>,
    pending_effects: VecDeque<Effect>,
    pending_updates: usize,
    entities: HashMap<EntityId, Box<RefCell<dyn Any>>>,
    flushing_effects: bool,
    event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    release_listeners: SubscriberSet<EntityId, ReleaseListener>,
}

// #[cfg(test)]
// mod tests {
//     use tokio::{task::spawn_local, time::Duration};

//     use super::*;

//     #[test]
//     fn test_basic() {
//         let ctx = Context::new();

//         dbg!(&ctx.test);

//         one(&ctx);

//         ctx.spawn_local(move |ctx| async move {
//             // two(ctx);

//             // dbg!(&(*ctx.get()).test);

//             tokio::time::sleep(Duration::from_secs(3)).await
//         });

//         std::thread::sleep(std::time::Duration::from_secs(5));

//         dbg!(&ctx.test);
//     }

//     fn one(ctx: &Context) {
//         dbg!(1);
//     }

//     // fn two(ctx: Context) {
//     //     let mut test_lock = &mut ctx.test.lock();

//     //     test_lock = "update".to_string();
//     // }
// }
