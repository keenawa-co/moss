use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, Stream};

use crate::domain::model::{
    notification::Notification,
    result::{Result, ResultExtension},
};

#[derive(Debug)]
pub struct NotificationService {
    tx: mpsc::Sender<Notification>,
    rx: Arc<RwLock<mpsc::Receiver<Notification>>>,
}

impl NotificationService {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Notification>(100);
        Self {
            tx,
            rx: Arc::new(RwLock::new(rx)),
        }
    }

    pub async fn send(&self, v: Notification) -> Result<()> {
        Ok(self
            .tx
            .send(v)
            .await
            .ok_or_system_unexpected("Failed to send message", None)?)
    }

    pub async fn subscribe(&self) -> impl Stream<Item = Notification> {
        let (tx, rx) = mpsc::channel(32);
        let rx_clone = self.rx.clone();

        tokio::spawn(async move {
            let mut receiver = rx_clone.write().await;
            while let Some(value) = receiver.recv().await {
                tx.send(value).await.unwrap();
            }
        });

        ReceiverStream::new(rx)
    }
}
