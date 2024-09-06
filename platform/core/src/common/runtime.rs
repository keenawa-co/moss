use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use super::context::ContextInner;

#[derive(Deref, DerefMut)]
pub struct AsyncRuntime(ContextInner);

impl AsyncRuntime {
    pub fn new() -> Self {
        Self(ContextInner::new())
    }

    pub fn run<F, R>(self, f: F) -> R
    where
        F: 'static + FnOnce(ContextInner) -> R,
    {
        f(self.0)
    }
}
