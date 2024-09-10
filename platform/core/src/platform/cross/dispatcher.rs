use async_task::Runnable;
use flume::Sender;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use crate::platform::AnyDispatcher;

#[derive(Debug)]
pub struct Dispatcher {
    parker: Mutex<Parker>,
    main_sender: Sender<Runnable>,
    background_sender: Sender<Runnable>,
    _background_threads: Arc<Vec<thread::JoinHandle<()>>>,
}

impl AnyDispatcher for Dispatcher {
    fn park(&self, timeout: Option<Duration>) -> bool {
        if let Some(t) = timeout {
            self.parker.lock().park_timeout(t)
        } else {
            self.parker.lock().park();
            true
        }
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }

    fn dispatch(&self, runnable: Runnable) {
        self.background_sender.send(runnable).unwrap();
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender.send(runnable).unwrap();
    }
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
            parker: Mutex::new(Parker::new()),
            main_sender,
            background_sender,
            _background_threads: Arc::new(background_threads),
        }
    }
}
