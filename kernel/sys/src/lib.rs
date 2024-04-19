use anyhow::anyhow;
use chan::BroadcastChannel;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

pub type Config = notify::Config;

pub struct FileWatcher {
    channel: Arc<BroadcastChannel<f32>>,
    watcher: Mutex<Option<RecommendedWatcher>>,
    subscriber_count: Mutex<usize>,
    watch_list: Mutex<Vec<PathBuf>>,
}

impl FileWatcher {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            channel: Arc::new(BroadcastChannel::new(32)),
            watcher: Mutex::new(None),
            subscriber_count: Mutex::new(0),
            watch_list: Mutex::new(Vec::new()),
        })
    }

    pub fn subscribe(self: &Arc<Self>) -> anyhow::Result<broadcast::Receiver<f32>> {
        let mut subscriber_count = self
            .subscriber_count
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?;
        *subscriber_count += 1;

        // run the watcher if this is the first subscriber
        if *subscriber_count == 1 {
            drop(subscriber_count);
            self.run()?;
        }

        Ok(self.channel.subscribe())
    }

    pub fn unsubscribe(self: &Arc<Self>) -> anyhow::Result<()> {
        let mut subscriber_count = self
            .subscriber_count
            .lock()
            .map_err(|e| anyhow!(e.to_string()))?;
        if *subscriber_count > 0 {
            *subscriber_count -= 1;
        }

        // stop the watcher if this is the last subscriber
        if *subscriber_count == 0 {
            self.stop()?;
        }

        Ok(())
    }

    pub fn watch_path(self: &Arc<Self>, path: &PathBuf) -> anyhow::Result<()> {
        let mut list_lock = self.watch_list.lock().map_err(|e| anyhow!(e.to_string()))?;
        if !list_lock.contains(path) {
            list_lock.push(path.clone());
            self.create_watcher_path(path)?;
        }

        Ok(())
    }

    fn create_watcher(&self) -> anyhow::Result<RecommendedWatcher> {
        let chan = self.channel.clone();
        let watcher = RecommendedWatcher::new(
            move |res| {
                futures::executor::block_on(async {
                    match res {
                        Ok(event) => {
                            println!("Event detected: {:?}", event);

                            if let Err(e) = chan.send(0.5) {
                                eprintln!("Error sending event: {:?}", e);
                            }
                        }
                        Err(e) => eprintln!("Watch error: {:?}", e),
                    }
                });
            },
            Config::default(),
        )?;

        Ok(watcher)
    }

    fn create_watcher_path(&self, path: &PathBuf) -> anyhow::Result<()> {
        let mut watcher_lock = self.watcher.lock().map_err(|e| anyhow!(e.to_string()))?;
        if let Some(ref mut watcher) = *watcher_lock {
            watcher.watch(path, RecursiveMode::Recursive)?;
        }

        Ok(())
    }

    fn run(self: &Arc<Self>) -> anyhow::Result<()> {
        let mut watcher_lock = self.watcher.lock().map_err(|e| anyhow!(e.to_string()))?;
        if watcher_lock.is_none() {
            *watcher_lock = Some(self.create_watcher()?);
        }

        let paths = self.watch_list.lock().map_err(|e| anyhow!(e.to_string()))?;
        for path in paths.iter() {
            self.create_watcher_path(&path)?;
        }

        Ok(())
    }

    fn stop(self: &Arc<Self>) -> anyhow::Result<()> {
        let paths = self.watch_list.lock().map_err(|e| anyhow!(e.to_string()))?;
        let mut watcher_lock = self.watcher.lock().map_err(|e| anyhow!(e.to_string()))?;

        if let Some(watcher) = watcher_lock.as_mut() {
            for path in paths.iter() {
                watcher.unwatch(&path)?;
            }
        }

        Ok(())
    }
}
