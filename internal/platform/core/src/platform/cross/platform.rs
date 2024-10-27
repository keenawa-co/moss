use std::sync::Arc;

use async_task::Runnable;
use flume::Receiver;

use crate::{
    executor::{BackgroundExecutor, MainThreadExecutor},
    platform::AnyDispatcher,
};

use super::dispatcher::Dispatcher;

pub struct CrossPlatform {
    pub(super) main_thread_executor: MainThreadExecutor,
    pub(super) background_executor: BackgroundExecutor,
}

impl CrossPlatform {
    pub fn new() -> (Self, Receiver<Runnable>) {
        let (main_tx, main_rx) = flume::unbounded::<Runnable>();
        let dispatcher = Arc::new(Dispatcher::new(main_tx));

        (
            Self {
                main_thread_executor: MainThreadExecutor::new(
                    Arc::clone(&dispatcher) as Arc<dyn AnyDispatcher>
                ),
                background_executor: BackgroundExecutor::new(
                    Arc::clone(&dispatcher) as Arc<dyn AnyDispatcher>
                ),
            },
            main_rx,
        )
    }
}
