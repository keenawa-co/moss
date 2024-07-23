use async_task::{Runnable, Task};
use std::fmt::Debug;
use std::future::Future;
use std::panic::catch_unwind;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use super::QueueBackend;

#[derive(Debug)]
pub struct ThreadBackend {
    work_tx: flume::Sender<Runnable>,
    quit_tx: flume::Sender<()>,
    status: Arc<AtomicBool>,
}

impl ThreadBackend {
    pub fn new() -> Self {
        let (work_tx, work_rx) = flume::unbounded::<Runnable>();
        let (quit_tx, quit_rx) = flume::unbounded::<()>();
        let status = Arc::new(AtomicBool::new(false));

        let status_flag = Arc::clone(&status);
        thread::spawn(move || {
            status_flag.store(true, Ordering::SeqCst);

            loop {
                flume::Selector::new()
                    .recv(&quit_rx, |_| {
                        status_flag.store(false, Ordering::SeqCst);
                        None
                    })
                    .recv(&work_rx, |msg| {
                        if let Ok(runnable) = msg {
                            // Ignore panics inside futures.
                            let _ignore_panic = catch_unwind(|| runnable.run());
                        }
                        Some(())
                    })
                    .wait();

                if !status_flag.load(Ordering::SeqCst) {
                    break;
                }
            }
        });

        Self {
            work_tx,
            quit_tx,
            status,
        }
    }
}

#[async_trait]
impl QueueBackend for ThreadBackend {
    fn spawn<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let sender = self.work_tx.clone();
        let schedule = move |runnable| sender.send(runnable).unwrap();
        let (runnable, task) = async_task::spawn(future, schedule);

        runnable.schedule();

        task
    }

    fn stop(&self) {
        self.quit_tx.send(()).unwrap();
    }

    fn status(&self) -> bool {
        self.status.load(Ordering::SeqCst)
    }
}
