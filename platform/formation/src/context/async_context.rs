use derive_more::{Deref, DerefMut};
use std::{cell::RefCell, rc::Weak};

use super::context::PlatformContext;

#[derive(Deref, DerefMut)]
pub struct AsyncContext {
    #[deref]
    #[deref_mut]
    pub app: Weak<RefCell<PlatformContext>>,
}

unsafe impl Send for AsyncContext {}
unsafe impl Sync for AsyncContext {}
