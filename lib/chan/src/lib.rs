use tokio::sync::broadcast::{self, error::SendError};

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
