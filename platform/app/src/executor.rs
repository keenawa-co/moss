use std::{
    marker::PhantomData,
    num::NonZeroUsize,
    pin::Pin,
    rc::Rc,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    task::Poll,
    time::Duration,
};

use futures::Future;
use smol::future::FutureExt;
use waker_fn::waker_fn;

use crate::platform::PlatformDispatcher;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TaskLabel(NonZeroUsize);

impl Default for TaskLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskLabel {
    pub fn new() -> Self {
        static NEXT_TASK_LABEL: AtomicUsize = AtomicUsize::new(1);
        Self(
            NEXT_TASK_LABEL
                .fetch_add(1, Ordering::SeqCst)
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Debug)]
pub enum Task<T> {
    /// A task that is ready to return a value
    Ready(Option<T>),

    /// A task that is currently running.
    Spawned(async_task::Task<T>),
}

impl<T> Task<T> {
    /// Creates a new task that will resolve with the value
    pub fn ready(val: T) -> Self {
        Task::Ready(Some(val))
    }

    /// Detaching a task runs it to completion in the background
    pub fn detach(self) {
        match self {
            Task::Ready(_) => {}
            Task::Spawned(task) => task.detach(),
        }
    }
}

impl<T> Future for Task<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, ctx: &mut std::task::Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            Task::Ready(val) => Poll::Ready(val.take().unwrap()),
            Task::Spawned(task) => task.poll(ctx),
        }
    }
}

type AnyFuture<R> = Pin<Box<dyn 'static + Send + Future<Output = R>>>;
type AnyLocalFuture<R> = Pin<Box<dyn 'static + Future<Output = R>>>;

#[derive(Clone)]
pub struct BackgroundTaskExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
}

impl BackgroundTaskExecutor {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self { dispatcher }
    }

    pub fn spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.spawn_internal::<R>(Box::pin(future), None)
    }

    pub fn block_on<R>(&self, future: impl Future<Output = R>) -> R {
        if let Ok(value) = self.block_on_internal(future, None) {
            value
        } else {
            unreachable!()
        }
    }

    fn block_on_internal<R>(
        &self,
        future: impl Future<Output = R>,
        timeout: Option<Duration>,
    ) -> Result<R, impl Future<Output = R>> {
        use std::time::Instant;

        let mut future = Box::pin(future);
        if timeout == Some(Duration::ZERO) {
            return Err(future);
        }

        let deadline = timeout.map(|timeout| Instant::now() + timeout);
        let unparker = self.dispatcher.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        let mut ctx = std::task::Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut ctx) {
                Poll::Ready(result) => return Ok(result),
                Poll::Pending => {
                    let timeout =
                        deadline.map(|deadline| deadline.saturating_duration_since(Instant::now()));

                    if !self.dispatcher.park(timeout) {
                        if deadline.is_some_and(|deadline| deadline < Instant::now()) {
                            return Err(future);
                        }
                    }
                }
            }
        }
    }

    fn spawn_internal<R: Send + 'static>(
        &self,
        future: AnyFuture<R>,
        label: Option<TaskLabel>,
    ) -> Task<R> {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn(future, move |runnable| dispatcher.dispatch(runnable, label));
        runnable.schedule();
        Task::Spawned(task)
    }
}

#[derive(Clone)]
pub struct ForegroundTaskExecutor {
    dispatcher: Arc<dyn PlatformDispatcher>,
    not_send: PhantomData<Rc<()>>,
}

impl ForegroundTaskExecutor {
    pub fn new(dispatcher: Arc<dyn PlatformDispatcher>) -> Self {
        Self {
            dispatcher,
            not_send: PhantomData,
        }
    }

    pub fn spawn<R>(&self, future: impl Future<Output = R> + 'static) -> Task<R>
    where
        R: 'static,
    {
        let dispatcher = self.dispatcher.clone();
        fn inner<R: 'static>(
            dispatcher: Arc<dyn PlatformDispatcher>,
            future: AnyLocalFuture<R>,
        ) -> Task<R> {
            let (runnable, task) = async_task::spawn_local(future, move |runnable| {
                println!("Scheduling runnable on main thread");

                dispatcher.dispatch_on_main_thread(runnable)
            });
            runnable.schedule();
            Task::Spawned(task)
        }

        inner::<R>(dispatcher, Box::pin(future))
    }
}

// --------------------

use std::task::Context;
use tokio::task;

#[derive(Debug)]
pub enum TaskCompact<T> {
    Ready(Option<T>),
    Spawned(task::JoinHandle<T>),
}

impl<T> TaskCompact<T> {
    pub fn ready(val: T) -> Self {
        TaskCompact::Ready(Some(val))
    }

    pub fn abort(self) {
        if let TaskCompact::Spawned(task) = self {
            task.abort();
        }
    }
}

impl<T> Future for TaskCompact<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            TaskCompact::Ready(val) => Poll::Ready(val.take().unwrap()),
            TaskCompact::Spawned(task) => match Pin::new(task).poll(ctx) {
                Poll::Ready(Ok(val)) => Poll::Ready(val),
                Poll::Pending => Poll::Pending,
                Poll::Ready(Err(e)) => panic!("Task failed: {:?}", e),
            },
        }
    }
}
