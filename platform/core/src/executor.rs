use futures::Future;
use smol::future::FutureExt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{pin::Pin, task::Poll};
use waker_fn::waker_fn;

use crate::platform::AnyDispatcher;

pub enum Task<T> {
    Ready(Option<T>),
    Spawned(async_task::Task<T>),
}

impl<T> Task<T> {
    pub fn ready(val: T) -> Self {
        Task::Ready(Some(val))
    }

    pub fn detach(self) {
        match self {
            Task::Ready(_) => {}
            Task::Spawned(task) => task.detach(),
        }
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            Task::Ready(val) => Poll::Ready(val.take().unwrap()),
            Task::Spawned(task) => task.poll(cx),
        }
    }
}

#[derive(Clone)]
pub struct BackgroundExecutor {
    dispatcher: Arc<dyn AnyDispatcher>,
}

type AnyFuture<R> = Pin<Box<dyn 'static + Send + Future<Output = R>>>;
type AnyLocalFuture<R> = Pin<Box<dyn 'static + Future<Output = R>>>;

impl BackgroundExecutor {
    pub fn new(dispatcher: Arc<dyn AnyDispatcher>) -> Self {
        BackgroundExecutor { dispatcher }
    }

    pub fn block_on<R>(&self, fut: impl Future<Output = R>) -> R {
        if let Ok(value) = self.block_on_internal(fut, None) {
            value
        } else {
            unreachable!()
        }
    }

    pub(crate) fn block_on_internal<R>(
        &self,
        future: impl Future<Output = R>,
        timeout: Option<Duration>,
    ) -> Result<R, impl Future<Output = R>> {
        let mut future = Box::pin(future);
        if timeout == Some(Duration::ZERO) {
            return Err(future);
        }
        let deadline = timeout.map(|timeout| Instant::now() + timeout);

        let unparker = self.dispatcher.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        let mut cx = std::task::Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return Ok(result),
                Poll::Pending => {
                    let timeout =
                        deadline.map(|deadline| deadline.saturating_duration_since(Instant::now()));
                    if !self.dispatcher.park(timeout)
                        && deadline.is_some_and(|deadline| deadline < Instant::now())
                    {
                        return Err(future);
                    }
                }
            }
        }
    }

    pub fn spawn<R: Send + 'static>(
        &self,
        fut: impl Future<Output = R> + Send + 'static,
    ) -> Task<R> {
        self.spawn_internal::<R>(Box::pin(fut))
    }

    fn spawn_internal<R: Send + 'static>(&self, fut: AnyFuture<R>) -> Task<R> {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn(fut, move |runnable| dispatcher.dispatch(runnable));

        runnable.schedule();
        Task::Spawned(task)
    }
}

#[derive(Clone)]
pub struct MainThreadExecutor {
    dispatcher: Arc<dyn AnyDispatcher>,
    not_send: PhantomData<Rc<()>>,
}

impl MainThreadExecutor {
    pub fn new(dispatcher: Arc<dyn AnyDispatcher>) -> Self {
        Self {
            dispatcher,
            not_send: PhantomData::default(),
        }
    }

    pub fn spawn_local<R: 'static>(&self, fut: impl Future<Output = R> + 'static) -> Task<R> {
        let dispatcher = self.dispatcher.clone();
        fn inner<R: 'static>(
            dispatcher: Arc<dyn AnyDispatcher>,
            fut: AnyLocalFuture<R>,
        ) -> Task<R> {
            let (runnable, task) = async_task::spawn_local(fut, move |runnable| {
                dispatcher.dispatch_on_main_thread(runnable)
            });

            runnable.schedule();
            Task::Spawned(task)
        }

        inner::<R>(dispatcher, Box::pin(fut))
    }
}
