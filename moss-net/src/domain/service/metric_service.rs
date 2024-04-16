use std::sync::Arc;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct MetricService {
    pub tx: Arc<Sender<String>>,
    // pub repo
}

impl MetricService {
    pub fn new(tx: Arc<Sender<String>>) -> Self {
        Self { tx }
    }
}
