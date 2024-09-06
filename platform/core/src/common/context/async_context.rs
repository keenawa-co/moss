use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use super::Context;

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
