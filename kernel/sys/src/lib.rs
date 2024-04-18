use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;

pub struct Channel<T>
where
    T: Clone,
{
    tx: broadcast::Sender<T>,
}

impl<T> Channel<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Channel { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.tx.subscribe()
    }
}

pub struct FileWatcher {
    channel: Channel<f32>,
    watcher: Mutex<Option<RecommendedWatcher>>,
    status: Mutex<bool>,
    watch_list: Mutex<Vec<PathBuf>>,
}

impl FileWatcher {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            channel: Channel::new(32),
            watcher: Mutex::new(None),
            status: Mutex::new(false),
            watch_list: Mutex::new(Vec::new()),
        })
    }

    pub fn subscribe(self: &Arc<Self>) -> anyhow::Result<broadcast::Receiver<f32>> {
        println!("Subscribed!");

        let mut status = self.status.lock().unwrap();
        if !*status {
            *status = true;
            drop(status); // release lock before calling `run`
            self.run()?;
        }

        Ok(self.channel.subscribe())
    }

    pub fn register_path(self: &Arc<Self>, path: PathBuf) -> anyhow::Result<()> {
        let mut paths = self.watch_list.lock().unwrap();
        if !paths.contains(&path) {
            paths.push(path.clone());
        }

        // Automatically start watching new path if watcher is already running
        if *self.status.lock().unwrap() {
            self.add_path_to_watcher(&path)?;
        }

        Ok(())
    }

    fn add_path_to_watcher(&self, path: &PathBuf) -> anyhow::Result<()> {
        let mut watcher_guard = self.watcher.lock().unwrap();
        if let Some(watcher) = watcher_guard.as_mut() {
            watcher.watch(path, RecursiveMode::Recursive)?;
        }
        Ok(())
    }

    fn run(self: &Arc<Self>) -> anyhow::Result<()> {
        let initial_watch_list = self.watch_list.lock().unwrap().clone();
        let tx_clone = self.channel.tx.clone();

        let watcher = RecommendedWatcher::new(
            move |res| {
                futures::executor::block_on(async {
                    match res {
                        Ok(event) => {
                            println!("Event detected: {:?}", event);

                            if let Err(e) = tx_clone.send(0.5) {
                                eprintln!("Error sending event: {:?}", e);
                            }
                        }
                        Err(e) => eprintln!("Watch error: {:?}", e),
                    }
                });
            },
            Config::default(),
        )?;

        let mut watcher_lock = self.watcher.lock().unwrap();
        *watcher_lock = Some(watcher);

        for path in initial_watch_list {
            self.add_path_to_watcher(&path)?;
        }

        Ok(())
    }
}

// impl FileWatcher {
//     fn run(&mut self) -> anyhow::Result<()> {
//         let tx_clone = self.channel.tx.clone();

//         let watcher = RecommendedWatcher::new(
//             move |res| {
//                 dbg!(&res);

//                 futures::executor::block_on(async {
//                     match res {
//                         Ok(event) => {
//                             println!("Event detected: {:?}", event);
//                             let ev = EventType::FileEvent(FileEventType::ChangedEvent(
//                                 ChangedEventPayload { testdata: 0.5 },
//                             ));

//                             if let Err(e) = tx_clone.send(ev).await {
//                                 eprintln!("Error sending event: {:?}", e);
//                             }
//                         }
//                         Err(e) => eprintln!("Watch error: {:?}", e),
//                     }
//                 })
//             },
//             notify::Config::default(),
//         )?;

//         self.watcher = Some(watcher);

//         Ok(())
//     }
// }
