use async_task::{Runnable, Task};
use once_cell::sync::Lazy;
use std::future::Future;
use std::panic::catch_unwind;

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[async_trait]
pub trait QueueBackend {
    fn spawn<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;
}

#[derive(Debug)]
pub struct ThreadBackend {
    tx: Lazy<flume::Sender<Runnable>>,
}

impl ThreadBackend {
    pub fn new() -> Self {
        let sender: Lazy<flume::Sender<Runnable>> = Lazy::new(move || {
            let (sender, receiver) = flume::unbounded::<Runnable>();
            let (stop_tx, stop_rx) = flume::unbounded::<()>();
            let stop_flag = AtomicBool::new(false);

            thread::spawn(move || {
                loop {
                    flume::Selector::new()
                        .recv(&stop_rx, |_| {
                            stop_flag.store(true, Ordering::SeqCst);
                            None
                        })
                        .recv(&receiver, |msg| {
                            if let Ok(runnable) = msg {
                                // Ignore panics inside futures.
                                let _ignore_panic = catch_unwind(|| runnable.run());
                            }
                            Some(())
                        })
                        .wait();

                    if stop_flag.load(Ordering::SeqCst) {
                        break;
                    }
                }
            });

            sender
        });

        Self { tx: sender }
    }
}

#[async_trait]
impl QueueBackend for ThreadBackend {
    fn spawn<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let sender = self.tx.clone();
        let schedule = move |runnable| sender.send(runnable).unwrap();
        let (runnable, task) = async_task::spawn(future, schedule);

        runnable.schedule();

        task
    }
}

#[derive(Debug)]
pub struct Queue<B>
where
    B: QueueBackend,
{
    backend: B,
}

impl<B> Queue<B>
where
    B: QueueBackend,
{
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    pub fn enqueue<F, T>(&self, future: F)
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        self.backend.spawn(future).detach();
    }
}
