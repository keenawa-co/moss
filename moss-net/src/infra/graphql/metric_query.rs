use std::sync::Arc;

use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

use crate::domain::service::MetricService;

#[derive(Default)]
pub(super) struct MetricSubscription;

#[Subscription]
impl MetricSubscription {
    async fn metric_result_feed(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<String>>> {
        let scheduler = ctx.data::<Arc<MetricService>>()?;
        let receiver = scheduler.tx.subscribe();

        Ok(BroadcastStream::new(receiver).map(|res| {
            res.map_err(|e| async_graphql::Error::new(e.to_string()))
                .map(|v| v)
        }))
    }
}
