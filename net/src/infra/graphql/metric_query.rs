use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt};
use pe::policy::Report;
use std::sync::Arc;

use crate::domain::service::MetricService;

#[derive(Default)]
pub(super) struct MetricSubscription;

#[Subscription]
impl MetricSubscription {
    async fn metric_feed(
        &self,
        ctx: &Context<'_>,
        // _metric_list: Vec<String>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<Report>>> {
        let metric_service = ctx.data::<Arc<MetricService>>()?;
        let stream = metric_service
            .subscribe()?
            .map(|result| result.map_err(|e| async_graphql::Error::new(e.to_string())));

        Ok(stream)
    }
}
