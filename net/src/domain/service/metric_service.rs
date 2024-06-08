use std::sync::Arc;

#[derive(Clone)]
pub struct MetricService {}

impl MetricService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}
