use std::{path::PathBuf, sync::Arc};

use analysis::policy_engine::Report;
use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt};
use tokio::sync::RwLock;

use crate::domain::service::{metric_service::MetricService, project_service::ProjectService};

pub(super) struct MetricSubscription {
    pub metric_service: Arc<MetricService>,
    pub project_service: Arc<RwLock<ProjectService>>,
}

#[Subscription]
impl MetricSubscription {
    async fn metric_feed(
        &self,
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<Vec<String>>>> {
        let project_service_lock = self.project_service.write().await;
        let stream = project_service_lock
            .watch_project()
            .await
            .map(|list: Vec<PathBuf>| {
                let strings = list
                    .into_iter()
                    .map(|path_buf| path_buf.to_string_lossy().to_string())
                    .collect::<Vec<String>>();
                FieldResult::Ok(strings)
            });

        Ok(stream)
    }
}
