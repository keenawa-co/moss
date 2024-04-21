pub mod signal;

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task;

use crate::signal::Signal;

#[macro_use]
extern crate anyhow;

pub type Receiver = mpsc::Receiver<Signal>;
pub type Sender = mpsc::Sender<Signal>;

pub struct Dispatcher {
    rx: Arc<Mutex<Receiver>>,
    tx: Option<Sender>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);

        Self {
            rx: Arc::new(Mutex::new(rx)),
            tx: Some(tx),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<Sender> {
        if let Some(tx) = self.tx.take() {
            let rx_clone = self.rx.clone();

            task::spawn(async move {
                let mut rx_lock = rx_clone.lock().await;
                while let Some(message) = rx_lock.recv().await {
                    println!("Received event: {:?}", message);
                }
            });

            Ok(tx)
        } else {
            Err(anyhow!("dispatcher sender was already taken"))
        }
    }
}
