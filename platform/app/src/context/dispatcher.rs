use async_task::Runnable;
use parking_lot::Mutex;
use std::ptr::NonNull;
use std::thread;
use std::{sync::Arc, time::Duration};
use tokio::runtime::Runtime;
use tokio::sync::Notify;
use tokio::task;
use tracing::debug;

use super::task::TaskLabel;

// pub trait PlatformDispatcher: Send + Sync {
//     fn is_main_thread(&self) -> bool;
//     fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>);
//     fn dispatch_on_main_thread(&self, runnable: Runnable);
//     fn dispatch_after(&self, duration: Duration, runnable: Runnable);
//     fn park(&self, timeout: Option<Duration>) -> bool;
//     fn unparker(&self) -> Arc<Notify>;
// }

pub struct CrossPlatformDispatcher {
    runtime: Arc<Runtime>,
    notify: Arc<Notify>,
    main_thread_id: std::thread::ThreadId,
}

impl CrossPlatformDispatcher {
    pub fn new() -> Self {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        );
        let notify = Arc::new(Notify::new());
        let main_thread_id = std::thread::current().id();

        CrossPlatformDispatcher {
            runtime,
            notify,
            main_thread_id,
        }
    }
}

impl CrossPlatformDispatcher {
    fn is_main_thread(&self) -> bool {
        std::thread::current().id() == self.main_thread_id
    }

    // pub fn dispatch(&self, runnable: Runnable, label: Option<TaskLabel>) {
    //     let runtime = self.runtime.clone();
    //     runtime.spawn(async move {
    //         println!("Task is being executed");
    //         runnable.run();
    //         if let Some(label) = label {
    //             debug!("TaskLabel: {:?}", label);
    //         }
    //     });
    // }

    pub fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        thread::spawn(move || {
            let task = unsafe {
                Runnable::<()>::from_raw(NonNull::new_unchecked(
                    runnable.into_raw().as_ptr() as *mut ()
                ))
            };
            task.run();
        });
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        if self.is_main_thread() {
            runnable.run();
        } else {
            let runtime = self.runtime.clone();
            runtime.spawn(async move {
                task::spawn_blocking(move || {
                    runnable.run();
                })
                .await
                .expect("Blocking task failed");
            });
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        let runtime = self.runtime.clone();
        runtime.spawn(async move {
            tokio::time::sleep(duration).await;
            runnable.run();
        });
    }

    async fn park(&self, timeout: Option<Duration>) -> bool {
        let notify = self.notify.clone();
        if let Some(duration) = timeout {
            tokio::time::timeout(duration, notify.notified())
                .await
                .is_ok()
        } else {
            notify.notified().await;
            true
        }
    }

    fn unparker(&self) -> Arc<Notify> {
        self.notify.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_task::Runnable;
    use futures::SinkExt;
    use smol::future;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tokio::sync::oneshot;
    use tokio::time::Duration;

    use async_task::Task;

    fn create_runnable(flag: Arc<AtomicBool>) -> Runnable {
        let (runnable, task) = async_task::spawn(
            async move {
                flag.store(true, Ordering::Relaxed);
            },
            |task: Runnable| {
                task.run();
            },
        );
        runnable
    }

    #[test]
    fn test_is_main_thread() {
        let runtime = Runtime::new().unwrap();
        let _guard = runtime.enter();

        let dispatcher = CrossPlatformDispatcher::new();
        assert!(dispatcher.is_main_thread());
    }

    #[test]
    fn test() {
        let dispatcher = CrossPlatformDispatcher::new();

        println!("Hello!");

        let future = async {
            println!("Hello, world!");
        };

        // A function that schedules the task when it gets woken up.
        let (s, r) = flume::unbounded();
        let schedule = move |runnable| s.send(runnable).unwrap();

        // Create a task with the future and the schedule function.
        let (runnable, task) = async_task::spawn(future, schedule);

        dispatcher.dispatch(runnable, None);

        // smol::future::block_on(task);

        thread::sleep(Duration::from_secs(5));
    }
}
