use async_task::Runnable;
use derive_more::{Deref, DerefMut};
use futures::Future;
use smol::future::FutureExt;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use std::{pin::Pin, task::Context as TaskContext, task::Poll};
use tokio::runtime::Runtime;
use tokio::task;

pub enum ModernTask<T> {
    Ready(Option<T>),
    Spawned(async_task::Task<T>),
}

impl<T> ModernTask<T> {
    /// Creates a new task that will resolve with the value
    pub fn ready(val: T) -> Self {
        ModernTask::Ready(Some(val))
    }

    /// Detaching a task runs it to completion in the background
    pub fn detach(self) {
        match self {
            ModernTask::Ready(_) => {}
            ModernTask::Spawned(task) => task.detach(),
        }
    }
}

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

impl<T> Future for ModernTask<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
        match unsafe { self.get_unchecked_mut() } {
            ModernTask::Ready(val) => Poll::Ready(val.take().unwrap()),
            ModernTask::Spawned(task) => task.poll(cx),
        }
    }
}

use flume::{Receiver, Sender};

/// Local Executor that can spawn and manage local tasks.
#[derive(Clone)]
pub struct BackgroundExecutor {
    dispatcher: Dispatcher,
}

type AnyFuture<R> = Pin<Box<dyn 'static + Send + Future<Output = R>>>;
type AnyLocalFuture<R> = Pin<Box<dyn 'static + Future<Output = R>>>;

impl BackgroundExecutor {
    pub fn new(dispatcher: Dispatcher) -> Self {
        BackgroundExecutor { dispatcher }
    }

    pub fn spawn<R: Send + 'static>(
        &self,
        fut: impl Future<Output = R> + Send + 'static,
    ) -> ModernTask<R> {
        self.spawn_internal::<R>(Box::pin(fut))
    }

    fn spawn_internal<R: Send + 'static>(&self, future: AnyFuture<R>) -> ModernTask<R> {
        let dispatcher = self.dispatcher.clone();
        let (runnable, task) =
            async_task::spawn_local(future, move |runnable| dispatcher.dispatch(runnable));

        runnable.schedule();
        ModernTask::Spawned(task)
    }
}

#[derive(Debug, Clone)]
pub struct LocalExecutor {
    dispatcher: Dispatcher,
    not_send: PhantomData<Rc<()>>,
}

impl LocalExecutor {
    pub fn new(dispatcher: Dispatcher) -> Self {
        Self {
            dispatcher,
            not_send: PhantomData::default(),
        }
    }

    pub fn spawn<R: 'static>(&self, fut: impl Future<Output = R> + 'static) -> ModernTask<R> {
        let dispatcher = self.dispatcher.clone();
        fn inner<R: 'static>(dispatcher: Dispatcher, fut: AnyLocalFuture<R>) -> ModernTask<R> {
            let (runnable, task) = async_task::spawn_local(fut, move |runnable| {
                dispatcher.dispatch_on_main_thread(runnable)
            });

            dbg!("spawn_local inner");

            runnable.schedule();
            ModernTask::Spawned(task)
        }

        inner::<R>(dispatcher, Box::pin(fut))
    }
}

#[derive(Debug, Clone)]
pub struct Dispatcher {
    main_sender: Sender<Runnable>,
    background_sender: Sender<Runnable>,
    _background_threads: Arc<Vec<thread::JoinHandle<()>>>,
}

impl Dispatcher {
    pub fn new(main_sender: Sender<Runnable>) -> Self {
        let (background_sender, background_receiver) = flume::unbounded::<Runnable>();
        let thread_count = std::thread::available_parallelism()
            .map(|i| i.get())
            .unwrap_or(1);

        let mut background_threads = (0..thread_count)
            .map(|i| {
                let receiver = background_receiver.clone();
                std::thread::spawn(move || {
                    for runnable in receiver {
                        let start = Instant::now();

                        runnable.run();

                        println!(
                            "background thread {}: ran runnable. took: {:?}",
                            i,
                            start.elapsed()
                        )
                    }
                })
            })
            .collect::<Vec<_>>();

        Self {
            main_sender,
            background_sender,
            _background_threads: Arc::new(background_threads),
        }
    }

    pub fn dispatch(&self, runnable: Runnable) {
        self.background_sender.send(runnable).unwrap();
    }

    pub fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender.send(runnable).unwrap();
    }
}

/// _______________

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
