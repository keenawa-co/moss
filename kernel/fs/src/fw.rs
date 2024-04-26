use bus::{
    message::{simple_message::SimpleMessage, MessageBody},
    Bus,
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::sync::{broadcast, broadcast::error::SendError};

pub struct BroadcastChannel<T>
where
    T: Clone,
{
    tx: broadcast::Sender<T>,
    capacity: usize,
}

impl<T> BroadcastChannel<T>
where
    T: Clone,
{
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        BroadcastChannel { tx, capacity }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<T> {
        self.tx.subscribe()
    }

    pub fn send(&self, value: T) -> Result<usize, SendError<T>> {
        self.tx.send(value)
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

pub type Config = notify::Config;
pub type Channel = BroadcastChannel<notify::Event>;
pub type Receiver = broadcast::Receiver<notify::Event>;

pub struct FileWatcher {
    channel: Arc<Channel>,
    watcher: Mutex<Option<RecommendedWatcher>>,
    subscriber_count: Mutex<usize>,
    watch_list: Mutex<Vec<PathBuf>>, // FIXME:
    bus: Arc<Bus>,
}

impl FileWatcher {
    pub fn new(bus: Arc<Bus>) -> Arc<Self> {
        Arc::new(Self {
            channel: Arc::new(BroadcastChannel::new(32)),
            watcher: Mutex::new(None),
            subscriber_count: Mutex::new(0),
            watch_list: Mutex::new(Vec::new()),
            bus, // disp_tx,
        })
    }

    pub fn subscribe(self: &Arc<Self>) -> anyhow::Result<Receiver> {
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
        if list_lock.contains(path) {
            return Ok(());
        }

        list_lock.push(path.clone());

        let mut watcher_lock = self.watcher.lock().map_err(|e| anyhow!(e.to_string()))?;
        if watcher_lock.is_some() {
            let watcher = watcher_lock.as_mut().unwrap();
            watcher.watch(path, RecursiveMode::Recursive)?;
        }

        Ok(())
    }

    fn create_watcher(&self) -> anyhow::Result<RecommendedWatcher> {
        let chan = self.channel.clone();
        // let disp_tx_clone = self.disp_tx.clone();
        let watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                futures::executor::block_on(async {
                    match res {
                        Ok(event) => {
                            println!("Event detected: {:?}", event);

                            if let Err(e) = chan.send(event) {
                                eprintln!("Error sending event: {:?}", e);
                            }

                            // match event.kind {
                            //     // notify::EventKind::Any => todo!(),
                            //     // notify::EventKind::Access(_) => todo!(),
                            //     // notify::EventKind::Create(_) => todo!(),
                            //     notify::EventKind::Modify(kind) => match kind {
                            //         notify::event::ModifyKind::Any => todo!(),
                            //         notify::event::ModifyKind::Data(v) => {
                            //             let s = Signal::new(SignalType::File(FileSignal::Modify(
                            //                 event.paths,
                            //             )));

                            //             if let Err(e) = disp_tx_clone.send(s.clone()).await {
                            //                 eprintln!("Error sending event to dispatcher: {:?}", e);
                            //             }

                            //             if let Err(e) = chan.send(s) {
                            //                 eprintln!("Error sending event: {:?}", e);
                            //             }
                            //         }
                            //         _ => (),
                            //         // notify::event::ModifyKind::Metadata(_) => todo!(),
                            //         // notify::event::ModifyKind::Name(_) => todo!(),
                            //         // notify::event::ModifyKind::Other => todo!(),
                            //     },
                            //     // notify::EventKind::Remove(_) => todo!(),
                            //     // notify::EventKind::Other => todo!(),
                            //     _ => (),
                            // }
                        }
                        Err(e) => eprintln!("Watch error: {:?}", e),
                    }
                });
            },
            Config::default(),
        )?;

        Ok(watcher)
    }

    fn run(self: &Arc<Self>) -> anyhow::Result<()> {
        let mut watcher_lock = self.watcher.lock().map_err(|e| anyhow!(e.to_string()))?;
        if watcher_lock.is_none() {
            *watcher_lock = Some(self.create_watcher()?);
        }

        let watcher = watcher_lock.as_mut().unwrap();
        let paths = self.watch_list.lock().map_err(|e| anyhow!(e.to_string()))?;
        for path in paths.iter() {
            watcher.watch(path, RecursiveMode::Recursive)?;
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

#[async_trait]
impl bus::Consumer for FileWatcher {
    fn process(&self, _topic_name: &str, message: &SimpleMessage) {
        let r = message.body::<String>().unwrap();

        println!("message: {}", r)
    }
}

#[async_trait]
impl bus::Producer for FileWatcher {
    async fn publish(&self, topic_name: &str, message: SimpleMessage) -> anyhow::Result<()> {
        Ok(self.bus.publish(topic_name, message).await?)
    }
}

#[async_trait]
impl bus::Subscriber for FileWatcher {}
