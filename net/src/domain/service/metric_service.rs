use std::{path::PathBuf, pin::Pin, sync::Arc};

use analysis::{metric_engine::Report, policy_engine::PolicyEngine};
use fs::fw::FileWatcher;
use futures::Stream;
use tokio::sync::broadcast;

use crate::domain;

#[derive(Clone)]
pub struct MetricService {
    policy_engine: Arc<PolicyEngine>,
}

impl MetricService {
    pub fn new(analyzer: Arc<PolicyEngine>) -> Self {
        Self {
            policy_engine: analyzer,
        }
    }

    pub async fn subscribe(
        &self,
    ) -> anyhow::Result<Pin<Box<dyn Stream<Item = anyhow::Result<Report>> + Send>>> {
        self.policy_engine
            .register_watch_list(vec!["./testdata/helloworld.ts"])?;

        self.policy_engine.subscribe().await
    }
}
