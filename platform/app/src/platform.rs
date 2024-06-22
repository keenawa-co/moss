#[cfg(target_os = "macos")]
mod mac;

use async_task::Runnable;
use parking::Unparker;
use std::rc::Rc;
use std::time::Duration;

use crate::executor::{BackgroundTaskExecutor, ForegroundTaskExecutor, TaskLabel};

#[cfg(target_os = "macos")]
pub(crate) fn current_platform() -> Rc<dyn Platform> {
    Rc::new(mac::platform::MacPlatform::new())
}

pub trait Platform: 'static {
    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>);
    fn quit(&self);

    fn background_task_executor(&self) -> BackgroundTaskExecutor;
    fn foreground_task_executor(&self) -> ForegroundTaskExecutor;
}

pub trait PlatformDispatcher: Send + Sync {
    fn is_main_thread(&self) -> bool;
    fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>);
    fn dispatch_on_main_thread(&self, runnable: Runnable);
    fn dispatch_after(&self, duration: Duration, runnable: Runnable);
    fn park(&self, timeout: Option<Duration>) -> bool;
    fn unparker(&self) -> Unparker;
}
