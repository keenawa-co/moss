use anyhow::Result;
use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, future::Future, rc::Weak};

use super::{entity::Model, model_context::ModelContext, AnyContext, Context};
use crate::executor::{BackgroundExecutor, MainThreadExecutor, Task};

#[derive(Deref, DerefMut, Clone)]
pub struct AsyncContext {
    #[deref]
    #[deref_mut]
    pub(super) cell: Weak<RefCell<Context>>,
    pub(super) background_executor: BackgroundExecutor,
    pub(super) main_thread_executor: MainThreadExecutor,
}

unsafe impl Send for AsyncContext {}
unsafe impl Sync for AsyncContext {}

impl AsyncContext {
    pub fn update<R>(&self, f: impl FnOnce(&mut Context) -> R) -> Result<R> {
        let cell_mut = self
            .cell
            .upgrade()
            .ok_or_else(|| anyhow!("context was released"))?;

        let mut ctx = cell_mut.borrow_mut();
        Ok(f(&mut ctx))
    }

    pub fn new_model<T: 'static>(
        &self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Result<Model<T>> {
        let cell = self.upgrade().ok_or_else(|| anyhow!("app was released"))?;
        let mut ctx = cell.borrow_mut();
        Ok(ctx.new_model(build_model))
    }

    pub fn update_model<T: 'static, R>(
        &self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Result<R> {
        let cell = self
            .upgrade()
            .ok_or_else(|| anyhow!("context was released"))?;
        let mut ctx = cell.borrow_mut();
        Ok(ctx.update_model(handle, update))
    }

    pub fn block_on_with<Fut>(&self, f: impl FnOnce(AsyncContext) -> Fut) -> Fut::Output
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        self.background_executor.block_on(f(self.clone()))
    }

    pub fn block_on<R>(&self, fut: impl Future<Output = R>) -> R {
        self.background_executor.block_on(fut)
    }

    pub fn spawn_local<Fut>(&self, f: impl FnOnce(AsyncContext) -> Fut) -> Task<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        self.main_thread_executor.spawn_local(f(self.clone()))
    }
}
