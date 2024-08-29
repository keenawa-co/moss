use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use super::context::Context;

#[derive(Deref, DerefMut)]
pub struct AsyncRuntime(Arc<Mutex<Context>>);

impl AsyncRuntime {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Context::new())))
    }

    pub fn run<F>(self, f: F)
    where
        F: 'static + FnOnce(Arc<Mutex<Context>>),
    {
        f(self.0);
    }
}
