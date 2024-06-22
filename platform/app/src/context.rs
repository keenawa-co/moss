pub mod dispatcher;
pub mod event;
pub mod event_registry;
pub mod hook;

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use event::Event;
use event_registry::EventRegistry;
use futures::Future;
use parking_lot::RwLock;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    rc::{Rc, Weak},
    sync::Arc,
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
        println!("borrowed");
        AppRef(self.app.borrow())
    }

    pub fn borrow_mut(&self) -> AppRefMut {
        println!("borrowed mut");
        AppRefMut(self.app.borrow_mut())
    }
}

#[derive(Clone)]
pub struct AsyncAppContext {
    pub(crate) app: Weak<AppCell>,
    pub(crate) event_registry: EventRegistry,
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

    // pub fn with_event_registry_mut<T>(&self, f: impl FnOnce(&mut EventRegistry) -> T) -> T {
    //     let mut event_registry = self.update(|ctx: &mut AppContext| {});
    //     f(&mut event_registry)
    // }

    pub fn register_hook<E>(
        &self,
        hook_fn: impl Fn(&mut E) -> Result<()> + Send + Sync,
    ) -> Result<()>
    where
        E: Event + 'static,
    {
        self.update(|ctx: &mut AppContext| {
            // ctx.event_registry.register_hook(hook_fn);
        })
    }

    pub fn register_event<E>(&self) -> Result<()>
    where
        E: Event + 'static,
    {
        self.update(|ctx: &mut AppContext| {
            ctx.event_registry.register_event::<E>();
        })
    }

    pub fn notify<E>(&self, event: E) -> Task<()>
    where
        E: Event + Send + Sync + 'static,
    {
        // let future = {
        //     let r_clone = self.event_registry.clone();
        //     let event_clone = event.clone();
        //     async move {
        //         let event_registry = r_clone.read();
        //         // event_registry.dispatch_event(event_clone).await

        //         event_registry.te().await;
        //     }
        // };

        let r_clone = self.event_registry.clone();

        // self.background_task_executor.spawn(future).detach();
        self.spawn(move |_| {
            // let event_clone = event.clone();
            // let event_registry = r_clone.read();

            async move {
                let event_registry = r_clone.clone();
                event_registry.dispatch_event(event).await;
            }
        })
    }
}

#[derive(Clone)]
pub struct AppContext {
    pub(crate) this: Weak<AppCell>,
    pub(crate) platform: Rc<dyn Platform>,
    pub(crate) event_registry: EventRegistry,
    pub(crate) background_task_executor: BackgroundTaskExecutor,
    pub(crate) foreground_task_executor: ForegroundTaskExecutor,
}

impl AppContext {
    pub fn new(platform: Rc<dyn Platform>) -> Rc<AppCell> {
        Rc::new_cyclic(|this| AppCell {
            app: RefCell::new(AppContext {
                this: this.clone(),
                platform: platform.clone(),
                event_registry: EventRegistry::new(),
                background_task_executor: platform.background_task_executor(),
                foreground_task_executor: platform.foreground_task_executor(),
            }),
        })
    }

    pub fn register_hook<E>(&mut self, hook_fn: impl Fn(&mut E) -> Result<()> + Send + Sync)
    where
        E: Event + 'static,
    {
        self.event_registry.register_hook(hook_fn)
    }

    pub fn register_event<E>(&mut self)
    where
        E: Event + 'static,
    {
        self.event_registry.register_event::<E>();
    }

    pub fn into_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: self.this.clone(),
            event_registry: self.event_registry.clone(),
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

// pub fn with_event_registry<T>(&self, f: impl FnOnce(&EventRegistry) -> T) -> T {
//     f(&self.event_registry.read())
// }
