use async_graphql::{Context, FieldResult, Subscription};
use futures::{Stream, StreamExt};
use std::sync::Arc;
use tokio_stream::wrappers::BroadcastStream;

use crate::domain::service::MetricService;

#[derive(Default)]
pub(super) struct MetricSubscription;

#[Subscription]
impl MetricSubscription {
    async fn metric_result_feed(
        &self,
        ctx: &Context<'_>,
        metric_list: Vec<String>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<f32>>> {
        let metric_service = ctx.data::<Arc<MetricService>>()?;
        let receiver = metric_service.subscribe()?;

        Ok(BroadcastStream::new(receiver).map(move |res| {
            res.map_err(|e| async_graphql::Error::new(e.to_string()))
                .map(|v| v)
        }))
    }
}
