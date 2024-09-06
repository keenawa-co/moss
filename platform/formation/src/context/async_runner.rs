use derive_more::{Deref, DerefMut};
use futures::Future;
use std::{pin::Pin, task::Context as TaskContext, task::Poll};
use tokio::runtime::Runtime;
use tokio::task;

#[derive(Debug)]
pub enum Task<T> {
    Ready(Option<T>),
    Spawned(task::JoinHandle<T>),
}

impl<T> Task<T> {
    pub fn ready(val: T) -> Self {
        Task::Ready(Some(val))
    }

    pub fn abort(self) {
        if let Task::Spawned(task) = self {
            task.abort();
        }
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, task_ctx: &mut TaskContext) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            Task::Ready(val) => Poll::Ready(val.take().unwrap()),
            Task::Spawned(task) => match Pin::new(task).poll(task_ctx) {
                Poll::Ready(Ok(val)) => Poll::Ready(val),
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => panic!("Task failed: {:?}", e),
            },
        }
    }
}

#[derive(Deref, DerefMut)]
pub(super) struct Executor {
    #[deref]
    #[deref_mut]
    pub(crate) runtime: Runtime,
}

impl Executor {
    pub(super) fn new() -> Self {
        let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        else {
            panic!("failed to build async runtime");
        };

        Self { runtime }
    }
}
