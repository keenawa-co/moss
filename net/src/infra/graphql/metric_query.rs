use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt};
use graphql_utl::GraphQLExtendError;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::service::project_service::ProjectService;

pub(super) struct MetricSubscription<'a> {
    pub project_service: Arc<ProjectService<'a>>,
}

#[Subscription]
impl<'a> MetricSubscription<'a> {
    async fn metric_feed(
        &self,
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<Vec<String>>>> {
        let stream = self
            .project_service
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
