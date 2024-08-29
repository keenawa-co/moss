pub mod context;
pub mod entity;
pub mod model_context;
pub mod subscription;

use anyhow::Result;
use entity::{Model, Slot};
use model_context::ModelContext;

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

pub trait FlattenAnyhowResult<T> {
    fn flatten(self) -> Result<T>;
}

impl<T> FlattenAnyhowResult<T> for Result<Result<T>> {
    fn flatten(self) -> Result<T> {
        self?
    }
}

impl<T> FlattenAnyhowResult<T> for Result<T> {
    fn flatten(self) -> Result<T> {
        self
    }
}
