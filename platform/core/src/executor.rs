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

    pub fn block_on_internal<R>(
        &self,
        fut: impl Future<Output = R>,
        timeout: Option<Duration>,
    ) -> Result<R, impl Future<Output = R>> {
        let mut fut = Box::pin(fut);
        if timeout == Some(Duration::ZERO) {
            return Err(fut);
        }

        let deadline = timeout.map(|t| Instant::now() + t);
        let unparker = self.dispatcher.unparker();
        let awoken = Arc::new(AtomicBool::new(false));
        let waker = waker_fn({
            let awoken = awoken.clone();
            move || {
                awoken.store(true, Ordering::SeqCst);
                unparker.unpark();
            }
        });
        let mut ctx = std::task::Context::from_waker(&waker);

        loop {
            match fut.as_mut().poll(&mut ctx) {
                Poll::Ready(result) => return Ok(result),
                Poll::Pending => {
                    let timeout = deadline.map(|d| d.saturating_duration_since(Instant::now()));
                    if !self.dispatcher.park(timeout)
                        && deadline.is_some_and(|d| d < Instant::now())
                    {
                        return Err(fut);
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

    fn spawn_internal<R: Send + 'static>(&self, future: AnyFuture<R>) -> Task<R> {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn_local(future, move |runnable| dispatcher.dispatch(runnable));

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

    pub fn spawn<R: 'static>(&self, fut: impl Future<Output = R> + 'static) -> Task<R> {
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
