pub mod cross;

use async_task::Runnable;
use parking::Unparker;
use std::{future::Future, pin::Pin, process::ExitCode, time::Duration};

use crate::executor::{BackgroundExecutor, MainThreadExecutor};

pub trait AnyPlatform: 'static {
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
