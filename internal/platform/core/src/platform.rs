pub mod cross;

use async_task::Runnable;
use parking::Unparker;
use std::time::Duration;

use crate::executor::{BackgroundExecutor, MainThreadExecutor};

pub trait AnyPlatform: 'static {
    // TODO: run
    fn main_thread_executor(&self) -> MainThreadExecutor;
    fn background_executor(&self) -> BackgroundExecutor;
}

#[doc(hidden)]
pub trait AnyDispatcher: Send + Sync {
    fn dispatch(&self, runnable: Runnable);
    fn dispatch_on_main_thread(&self, runnable: Runnable);

    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
}
