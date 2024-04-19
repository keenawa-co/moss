use std::{path::PathBuf, sync::Arc};

use tokio::sync::broadcast;

use crate::domain;

#[derive(Clone)]
pub struct MetricService {
    fw: Arc<sys::FileWatcher>,
}

impl MetricService {
    pub fn new(fw: Arc<sys::FileWatcher>) -> Self {
        Self { fw }
    }

    pub fn subscribe(&self) -> domain::Result<broadcast::Receiver<f32>> {
        let rx = self.fw.subscribe()?;
        self.fw
            .watch_path(&PathBuf::from("./testdata/helloworld.ts"))?;

        Ok(rx)
    }
}
