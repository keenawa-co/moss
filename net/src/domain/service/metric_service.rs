use std::{path::PathBuf, pin::Pin, sync::Arc};

use fs::fw::FileWatcher;
use futures::Stream;
use pe::{engine::Engine as PolicyEngine, policy::Report};
use tokio::sync::broadcast;

use crate::domain;

#[derive(Clone)]
pub struct MetricService {
    policy_engine: Arc<PolicyEngine>,
}

impl MetricService {
    pub fn new(policy_engine: Arc<PolicyEngine>) -> Self {
        Self { policy_engine }
    }

    // pub fn subscribe(&self) -> domain::Result<broadcast::Receiver<f32>> {
    //     let rx = self.fw.subscribe()?;
    //     self.fw
    //         .watch_path(&PathBuf::from("./testdata/helloworld.ts"))?;

    //     Ok(rx)
    // }

    pub fn subscribe(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<Report>> + Send>>> {
        self.policy_engine
            .watch_path_list(vec!["./testdata/helloworld.ts"])?;

        self.policy_engine.run()
    }
}
