use analysis::{policy_engine::PolicyEngine, policy_engine::Report};
use std::{pin::Pin, sync::Arc};

use futures::Stream;

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
            .register_watch_list(vec!["./testdata/helloworld2.ts"])?;

        self.policy_engine.subscribe().await
    }
}
