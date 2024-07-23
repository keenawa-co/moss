pub mod thread_backend;

use async_task::Task;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

#[async_trait]
pub trait QueueBackend {
    fn spawn<F, T>(&self, future: F) -> Task<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static;

    fn stop(&self);
    fn status(&self) -> bool;
}

#[async_trait]
pub trait Processor<T>: Debug + Send + Sync + 'static
where
    T: Send + 'static,
{
    async fn process(&self, job: T);
}

#[derive(Debug)]
pub struct Queue<B, T>
where
    B: QueueBackend,
    T: Send + 'static,
{
    backend: Lazy<B>,
    processor: Arc<dyn Processor<T>>,
}

impl<B, T> Queue<B, T>
where
    B: QueueBackend,
    T: Send + 'static,
{
    pub fn new(backend: Lazy<B>, processor: impl Processor<T>) -> Self {
        Self {
            backend,
            processor: Arc::new(processor),
        }
    }

    pub fn enqueue(&self, job: T)
    where
        T: Send + 'static,
    {
        let processor = Arc::clone(&self.processor);

        self.backend
            .spawn(async move {
                processor.process(job).await;
            })
            .detach();
    }
}
