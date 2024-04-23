pub mod bus;
pub mod signal;

use anyhow::Context;
use signal::FileSignal;
use signal::SignalType;
use std::collections::HashMap;
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
}

impl Dispatcher {
    pub fn new() -> (Self, Sender) {
        let (tx, rx) = mpsc::channel(100);

        (
            Self {
                rx: Arc::new(Mutex::new(rx)),
                // analyzer_tx,
            },
            tx,
        )
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let rx_clone = self.rx.clone();
        // let analyzer_tx_clone = self.analyzer_tx.clone();

        task::spawn(async move {
            let mut rx_lock = rx_clone.lock().await;

            while let Some(signal) = rx_lock.recv().await {
                match &signal.typ {
                    SignalType::File(FileSignal::Modify(data)) => {
                        // analyzer_tx_clone.send(signal).await?
                    }
                    SignalType::File(FileSignal::Watch(_)) => todo!(),
                }
            }

            Ok::<(), anyhow::Error>(())
        });

        Ok(())
    }
}
