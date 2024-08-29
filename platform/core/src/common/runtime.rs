use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use super::context::context::Context;

#[derive(Deref, DerefMut)]
pub struct AsyncPlatformRuntime(Arc<Mutex<Context>>);

impl AsyncPlatformRuntime {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Context::new())))
    }

    pub fn exec<F>(self, f: F)
    where
        F: 'static + FnOnce(Arc<Mutex<Context>>),
    {
        f(self.0);
    }
}
