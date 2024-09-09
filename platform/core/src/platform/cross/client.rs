use async_task::Runnable;
use flume::Receiver;
use std::future::Future;
use std::pin::Pin;
use std::process::ExitCode;
use std::{cell::RefCell, rc::Rc};
use tokio::pin;
use tokio::task::LocalSet;

use crate::{
    executor::{BackgroundExecutor, MainThreadExecutor},
    platform::AnyPlatform,
};

use super::{config, platform::CrossPlatform};

// FIXME: Must be private.
// This must be reworked when platform-dependent clients are implemented.
pub struct CrossPlatformClientState {
    pub runtime: tokio::runtime::Runtime,
    pub local_set: LocalSet,
    pub platform: CrossPlatform,
    pub main_rx: Receiver<Runnable>,
}

impl CrossPlatformClientState {
    pub fn new(main_rx: Receiver<Runnable>, platform: CrossPlatform) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .max_blocking_threads(*config::RUNTIME_MAX_BLOCKING_THREADS)
            .thread_stack_size(*config::RUNTIME_STACK_SIZE)
            .build()
            .unwrap();

        let local_set = tokio::task::LocalSet::new();

        Self {
            runtime,
            local_set,
            platform,
            main_rx,
        }
    }
}

pub struct CrossPlatformClient(pub Rc<RefCell<CrossPlatformClientState>>);

impl CrossPlatformClient {
    pub fn new() -> Self {
        let (platform, main_rx) = CrossPlatform::new();
        let state = CrossPlatformClientState::new(main_rx, platform);

        Self(Rc::new(RefCell::new(state)))
    }

    // FIXME: Must be in AnyApp trait.
    pub fn run<Fut: Future>(&self, fut: Fut) -> Fut::Output {
        let state = self.0.as_ref().borrow();
        let main_rx_clone = state.main_rx.clone();
        dbg!(12);
        state.local_set.spawn_local(async move {
            while let Ok(runnable) = main_rx_clone.recv_async().await {
                dbg!(111);
                runnable.run();
            }
        });

        state.runtime.block_on(state.local_set.run_until(fut))
    }
}

impl AnyPlatform for CrossPlatformClient {
    fn main_thread_executor(&self) -> MainThreadExecutor {
        self.0
            .as_ref()
            .borrow()
            .platform
            .main_thread_executor
            .clone()
    }

    fn background_executor(&self) -> BackgroundExecutor {
        self.0
            .as_ref()
            .borrow()
            .platform
            .background_executor
            .clone()
    }
}
