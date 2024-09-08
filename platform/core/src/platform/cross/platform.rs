use crate::{
    executor::{BackgroundExecutor, MainThreadExecutor},
    platform::AnyPlatform,
};

pub(crate) struct CrossPlatformClient {
    main_thread_executor: MainThreadExecutor,
    background_executor: BackgroundExecutor,
}

impl AnyPlatform for CrossPlatformClient {
    fn main_thread_executor(&self) -> MainThreadExecutor {
        self.main_thread_executor.clone()
    }

    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }
}
