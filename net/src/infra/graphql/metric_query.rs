use std::sync::Arc;

use analysis::policy_engine::Report;
use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt};

use crate::domain::service::metric_service::MetricService;

pub(super) struct MetricSubscription {
    pub metric_service: Arc<MetricService>,
}

#[Subscription]
impl MetricSubscription {
    async fn metric_feed(
        &self,
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<Report>>> {
        let stream = self
            .metric_service
            .subscribe()
            .await?
            .map(|result| result.map_err(|e| async_graphql::Error::new(e.to_string())));

        Ok(stream)
    }
}
