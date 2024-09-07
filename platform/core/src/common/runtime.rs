use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::sync::Arc;

use super::context::Context;

// #[derive(Deref, DerefMut)]
// pub struct AsyncRuntime(Context);

// impl AsyncRuntime {
//     pub fn new() -> Self {
//         Self(Context::new())
//     }

//     pub fn run<F, R>(self, f: F) -> R
//     where
//         F: 'static + FnOnce(Context) -> R,
//     {
//         f(self.0)
//     }
// }
