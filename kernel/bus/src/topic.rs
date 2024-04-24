use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex, RwLock};

use crate::{message::simple_message::SimpleMessage, Consumer};

pub struct Topic {
    name: String,
    pub(crate) rx: Arc<RwLock<mpsc::Receiver<SimpleMessage>>>,
    pub(crate) tx: mpsc::Sender<SimpleMessage>,
    pub(crate) roster: Arc<RwLock<HashMap<TypeId, Arc<dyn Consumer>>>>,
    is_running: AtomicBool,
}

impl Topic {
    pub fn new(name: String) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Topic {
            name,
            tx,
            rx: Arc::new(RwLock::new(rx)),
            roster: Arc::new(RwLock::new(HashMap::new())),
            is_running: AtomicBool::new(false),
        }
    }

    pub async fn start(&self) {
        if !self.is_running.fetch_or(true, Ordering::SeqCst) {
            let rx_clone = self.rx.clone();
            let roster_clone = self.roster.clone();
            let topic_name_clone = self.name.clone(); // Сохраняем имя топика для использования в потребителях

            tokio::spawn(async move {
                let mut receiver = rx_clone.write().await;
                while let Some(message) = receiver.recv().await {
                    let roster = roster_clone.read().await;

                    if let Some(consumer) = roster.get(&message.type_id) {
                        consumer.process(&topic_name_clone, &message);
                    } else {
                        println!("No consumer found for this type of message");
                    }
                }
            });
        }
    }
}
