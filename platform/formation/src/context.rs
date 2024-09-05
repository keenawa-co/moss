pub mod entity;
pub mod subscription;

mod async_runner;

use async_runner::{Executor, Task};
use derive_more::{Deref, DerefMut};
use entity::{EntityId, EntityMap};
use moss_std::collection::{FxHashSet, VecDeque};
use parking_lot::Mutex;
use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    future::Future,
    sync::{Arc, Weak},
};
use subscription::SubscriberSet;

// #[derive(Deref, DerefMut)]
// pub struct AsyncContext {
//     cell: Weak<ContextCell>,
// }

#[derive(Deref, DerefMut, Clone)]
pub struct Context {
    #[deref]
    #[deref_mut]
    inner: Arc<ContextInner>,
    executor: Arc<Executor>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ContextInner {
                test: Mutex::new("initial".to_string()),
                rv: todo!(),
                executor: Executor::new(),
                observers: SubscriberSet::new(),
                pending_notifications: FxHashSet::default(),
                pending_effects: VecDeque::new(),
                pending_updates: 0,
                entities: EntityMap::new(),
                flushing_effects: false,
                event_listeners: SubscriberSet::new(),
                release_listeners: SubscriberSet::new(),
            }),
            executor: Arc::new(Executor::new()),
        }
    }

    pub fn spawn<Fut, R>(&self, f: impl FnOnce(Context) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        Task::Spawned(tokio::task::spawn_local(f(self.clone())))
    }
}

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
        callback: Box<dyn FnOnce(&mut ContextInner) + 'static>,
    },
}

type Handler = Box<dyn FnMut(&mut ContextInner) -> bool>;
type Listener = Box<dyn FnMut(&dyn Any, &mut ContextInner) -> bool>;
type ReleaseListener = Box<dyn FnOnce(&mut dyn Any, &mut ContextInner)>;

pub struct ContextInner {
    rv: flume::Receiver<Effect>,
    test: Mutex<String>,
    executor: Executor,
    observers: SubscriberSet<EntityId, Handler>,
    pending_notifications: FxHashSet<EntityId>,
    pending_effects: VecDeque<Effect>,
    pending_updates: usize,
    entities: EntityMap,
    flushing_effects: bool,
    event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,
    release_listeners: SubscriberSet<EntityId, ReleaseListener>,
}

// impl<'a> ContextInternal {
//     // pub fn defer(&'a mut self, f: impl FnOnce(&mut Context) + 'a) {
//     //     self.push_effect(Effect::Defer {
//     //         callback: Box::new(f),
//     //     })
//     // }
// }

// impl<'a> ContextInternal {
//     // pub fn push_effect(&'a mut self, effect: Effect<'a>) {
//     //     match &effect {
//     //         Effect::Notify { emitter } => {
//     //             if !self.pending_notifications.insert(*emitter) {
//     //                 return;
//     //             }
//     //         }
//     //         _ => {}
//     //     };

//     //     self.pending_effects.push_back(effect);
//     // }
// }

#[cfg(test)]
mod tests {
    use tokio::{task::spawn_local, time::Duration};

    use super::*;

    #[test]
    fn test_basic() {
        let ctx = Context::new();

        dbg!(&ctx.test);

        one(&ctx);

        ctx.spawn(move |ctx| async move {
            // two(ctx);

            // dbg!(&(*ctx.get()).test);

            tokio::time::sleep(Duration::from_secs(3)).await
        });

        std::thread::sleep(std::time::Duration::from_secs(5));

        dbg!(&ctx.test);
    }

    fn one(ctx: &Context) {
        dbg!(1);
    }

    // fn two(ctx: Context) {
    //     let mut test_lock = &mut ctx.test.lock();

    //     test_lock = "update".to_string();
    // }
}
