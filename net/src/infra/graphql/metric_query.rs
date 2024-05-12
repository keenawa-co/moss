use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt, TryStreamExt};
use graphql_utl::GraphQLExtendError;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::service::project_service::ProjectService;

pub(super) struct MetricSubscription {
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
            .extend_error()?
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
