use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use super::ContextInner;

#[derive(Deref, DerefMut)]
pub struct AsyncContext(Arc<Mutex<ContextInner>>);

impl From<ContextInner> for AsyncContext {
    fn from(value: ContextInner) -> Self {
        AsyncContext(Arc::new(Mutex::new(value)))
    }
}

impl AsyncContext {
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut ContextInner) -> R) -> R {
        f(&mut self.lock())
    }
}
