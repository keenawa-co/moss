use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use crate::common::context::Context;

#[derive(Deref, DerefMut)]
pub struct CommandAsyncContext(Arc<Mutex<Context>>);

impl From<Arc<Mutex<Context>>> for CommandAsyncContext {
    fn from(value: Arc<Mutex<Context>>) -> Self {
        CommandAsyncContext(value)
    }
}

impl CommandAsyncContext {
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut Context) -> R) -> R {
        f(&mut self.lock())
    }
}
