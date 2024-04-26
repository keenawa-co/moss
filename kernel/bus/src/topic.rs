use std::sync::atomic::{AtomicBool, Ordering};
use std::{any::TypeId, collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};

use crate::message::simple_message::SimpleMessage;
use crate::Subscriber;

pub struct TopicConfig {
    pub buffer: usize,
}

impl Default for TopicConfig {
    fn default() -> Self {
        Self { buffer: 100 }
    }
}

pub struct Topic {
    name: String,
    pub(crate) rx: Arc<RwLock<mpsc::Receiver<SimpleMessage>>>,
    pub(crate) tx: mpsc::Sender<SimpleMessage>,
    pub(crate) roster:
        Arc<RwLock<HashMap<TypeId, Arc<dyn Fn(&str, &SimpleMessage) + Send + Sync>>>>,
    status: AtomicBool,
}

impl Topic {
    pub fn new(name: String, config: TopicConfig) -> Self {
        let (tx, rx) = mpsc::channel(config.buffer);
        Topic {
            name,
            tx,
            rx: Arc::new(RwLock::new(rx)),
            roster: Arc::new(RwLock::new(HashMap::new())),
            status: AtomicBool::new(false),
        }
    }

    pub async fn start(&self) {
        if !self.status.fetch_or(true, Ordering::SeqCst) {
            let rx_clone = self.rx.clone();
            let roster_clone = self.roster.clone();
            let topic_name_clone = self.name.clone();

            tokio::spawn(async move {
                let mut receiver = rx_clone.write().await;
                while let Some(message) = receiver.recv().await {
                    let roster = roster_clone.read().await;

                    if let Some(consumer) = roster.get(&message.type_id) {
                        consumer(&topic_name_clone, &message);
                    } else {
                        println!("No consumer found for this type of message");
                    }
                }
            });
        }
    }
}
