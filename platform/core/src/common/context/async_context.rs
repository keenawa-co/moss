use anyhow::Result;
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::{cell::RefCell, future::Future, rc::Weak, sync::Arc};

use super::{
    async_runner::{BackgroundExecutor, LocalExecutor, ModernTask, Task},
    entity::Model,
    model_context::ModelContext,
    AnyContext, Context,
};

#[derive(Deref, DerefMut, Clone)]
pub struct ModernAsyncContext {
    #[deref]
    #[deref_mut]
    pub(super) cell: Weak<RefCell<Context>>,
    pub(super) background_executor: BackgroundExecutor,
    pub(super) local_executor: LocalExecutor,
}

unsafe impl Send for ModernAsyncContext {}
unsafe impl Sync for ModernAsyncContext {}

impl ModernAsyncContext {
    pub fn update<R>(&self, f: impl FnOnce(&mut Context) -> R) -> Result<R> {
        let cell_mut = self
            .cell
            .upgrade()
            .ok_or_else(|| anyhow!("context was released"))?;

        let mut ctx = cell_mut.borrow_mut();
        Ok(f(&mut ctx))
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

    pub fn spawn_local<Fut>(
        &self,
        f: impl FnOnce(ModernAsyncContext) -> Fut,
    ) -> ModernTask<Fut::Output>
    where
        Fut: Future + 'static,
        Fut::Output: 'static,
    {
        self.local_executor.spawn(f(self.clone()))
    }
}

// impl AnyContext for ModernAsyncContext {
//     type Result<T> = Result<T>;

//     fn reserve_model<T: 'static>(&mut self) -> Self::Result<super::Reservation<T>> {
//         todo!()
//     }

//     fn new_model<T: 'static>(
//         &mut self,
//         build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
//     ) -> Self::Result<Model<T>> {
//         let cell = self
//             .upgrade()
//             .ok_or_else(|| anyhow!("context was released"))?;
//         let mut ctx = cell.borrow_mut();
//         Ok(ctx.new_model(build_model))
//     }

//     fn insert_model<T: 'static>(
//         &mut self,
//         reservation: super::Reservation<T>,
//         build_model: impl FnOnce(&mut super::model_context::ModelContext<'_, T>) -> T,
//     ) -> Self::Result<super::entity::Model<T>> {
//         todo!()
//     }

//     fn update_model<T: 'static, R>(
//         &mut self,
//         handle: &Model<T>,
//         update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
//     ) -> Self::Result<R> {
//         let cell = self
//             .upgrade()
//             .ok_or_else(|| anyhow!("context was released"))?;
//         let mut ctx = cell.borrow_mut();
//         Ok(ctx.update_model(handle, update))
//     }
// }

#[derive(Deref, DerefMut)]
pub struct AsyncContext(Arc<Mutex<Context>>);

impl From<Context> for AsyncContext {
    fn from(value: Context) -> Self {
        AsyncContext(Arc::new(Mutex::new(value)))
    }
}

impl AsyncContext {
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut Context) -> R) -> R {
        f(&mut self.lock())
    }
}
