pub mod dispatcher;
pub mod event;
pub mod event_registry;
pub mod hook;
pub mod task;

use derive_more::{Deref, DerefMut};
use dispatcher::CrossPlatformDispatcher;
use event_registry::EventRegistry;
use futures::Future;
use parking_lot::RwLock;
use smol::future::FutureExt;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    pin::Pin,
    sync::{Arc, Weak},
    task::Poll,
};
use task::{Task, TaskLabel};

pub struct AppCell {
    pub app: RwLock<Context>,
}

#[derive(Deref, DerefMut)]
pub struct AppRef<'a>(parking_lot::RwLockReadGuard<'a, Context>);

#[derive(Deref, DerefMut)]
pub struct AppRefMut<'a>(parking_lot::RwLockWriteGuard<'a, Context>);

impl AppCell {
    pub fn borrow(&self) -> AppRef {
        AppRef(self.app.read())
    }

    pub fn borrow_mut(&self) -> AppRefMut {
        AppRefMut(self.app.write())
    }
}

pub struct AsyncContext {
    pub app: RwLock<Weak<AppCell>>,
}

impl AsyncContext {
    fn upgrade(&self) -> Option<Arc<AppCell>> {
        self.app.read().upgrade()
    }
}

type AnyFuture<R> = Pin<Box<dyn 'static + Send + Future<Output = R>>>;

#[derive(Clone)]
pub struct BackgroundExecutor {
    dispatcher: Arc<CrossPlatformDispatcher>,
}

impl BackgroundExecutor {
    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_internal::<R>(Box::pin(future), None)
    }

    pub fn spawn_labeled<R>(
        &self,
        label: TaskLabel,
        future: impl Future<Output = R> + Send + 'static,
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_internal::<R>(Box::pin(future), Some(label))
    }

    fn spawn_internal<R: Send + 'static>(
        &self,
        future: AnyFuture<R>,
        label: Option<TaskLabel>,
    ) -> Task<R> {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn(future, move |runnable| dispatcher.dispatch(runnable, label));
        runnable.schedule();
        Task::Spawned(task)
    }
}

pub struct Context {
    this: Weak<AppCell>,
    event_registry: RwLock<EventRegistry>,
    pub background_executor: BackgroundExecutor,
}

impl Context {
    pub fn new() -> Arc<AppCell> {
        let executor = BackgroundExecutor {
            dispatcher: Arc::new(CrossPlatformDispatcher::new()),
        };

        Arc::new_cyclic(|this| AppCell {
            app: RwLock::new(Context {
                this: Weak::clone(this),
                event_registry: RwLock::new(EventRegistry::new()),
                background_executor: executor,
            }),
        })
    }

    pub fn into_async(&self) -> AsyncContext {
        AsyncContext {
            app: RwLock::new(self.this.clone()),
        }
    }
}

// pub fn with_event_registry_mut<T>(&self, f: impl FnOnce(&mut EventRegistry) -> T) -> T {
//     f(&mut self.event_registry.write())
// }

// pub fn with_event_registry<T>(&self, f: impl FnOnce(&EventRegistry) -> T) -> T {
//     f(&self.event_registry.read())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test() {
//         use std::sync::atomic::{AtomicBool, Ordering};
//         use tokio::time::{sleep, Duration};

//         let task_executed = Arc::new(AtomicBool::new(false));
//         let task_executed_clone = task_executed.clone();

//         let context = Context::new();

//         let future = async move {
//             task_executed_clone.store(true, Ordering::SeqCst);
//         };

//         context.borrow().background_executor.spawn(future);
//     }
// }
