pub mod context;

pub mod platform;
pub mod service_registry;

use anyhow::Result;

#[macro_use]
extern crate anyhow;

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
