pub mod dispatcher;
pub mod event;
pub mod event_registry;
pub mod hook;

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use event_registry::EventRegistry;
use futures::Future;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
};

use crate::{
    executor::{BackgroundTaskExecutor, ForegroundTaskExecutor, Task},
    platform::Platform,
};

pub struct AppCell {
    app: RefCell<AppContext>,
}

#[derive(Deref, DerefMut)]
pub struct AppRef<'a>(Ref<'a, AppContext>);

#[derive(Deref, DerefMut)]
pub struct AppRefMut<'a>(RefMut<'a, AppContext>);

impl AppCell {
    pub fn borrow(&self) -> AppRef {
        AppRef(self.app.borrow())
    }

    pub fn borrow_mut(&self) -> AppRefMut {
        AppRefMut(self.app.borrow_mut())
    }
}

#[derive(Clone)]
pub struct AsyncAppContext {
    pub(crate) app: Weak<AppCell>,
    pub(crate) background_task_executor: BackgroundTaskExecutor,
}

unsafe impl Sync for AsyncAppContext {}
unsafe impl Send for AsyncAppContext {}

impl AsyncAppContext {
    pub fn update<R>(&self, f: impl FnOnce(&mut AppContext) -> R) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        Ok(f(&mut lock))
    }

    pub fn background_task_executor(&self) -> &BackgroundTaskExecutor {
        &self.background_task_executor
    }

    pub fn spawn<Fut, R>(&self, f: impl FnOnce(&AsyncAppContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        self.background_task_executor.spawn(f(&self))
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub(crate) this: Weak<AppCell>,
    pub(crate) platform: Rc<dyn Platform>,
    pub(crate) event_registry: Rc<EventRegistry>,
    pub(crate) background_task_executor: BackgroundTaskExecutor,
    pub(crate) foreground_task_executor: ForegroundTaskExecutor,
}

impl AppContext {
    pub fn new(platform: Rc<dyn Platform>) -> Rc<AppCell> {
        Rc::new_cyclic(|this| AppCell {
            app: RefCell::new(AppContext {
                this: this.clone(),
                platform: platform.clone(),
                event_registry: Rc::new(EventRegistry::new()),
                background_task_executor: platform.background_task_executor(),
                foreground_task_executor: platform.foreground_task_executor(),
            }),
        })
    }

    pub fn into_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: self.this.clone(),
            background_task_executor: self.background_task_executor.clone(),
        }
    }

    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        self.background_task_executor.spawn(f(self.into_async()))
    }

    pub fn block_on<'a, Fut, R>(&'a self, f: impl FnOnce(&'a AppContext) -> Fut) -> R
    where
        Fut: Future<Output = R>,
    {
        self.background_task_executor.block_on(f(self))
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
