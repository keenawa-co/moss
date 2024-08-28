use std::rc::Weak;

use super::context::ContextCell;

pub struct AsyncContext {
    pub app: std::sync::Weak<ContextCell>,
}

unsafe impl Send for AsyncContext {}
unsafe impl Sync for AsyncContext {}
