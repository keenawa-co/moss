use async_graphql::{Context, FieldResult, Subscription};
use graphql_utl::GraphQLExtendError;
use http::HeaderMap;
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

use crate::domain::service::notification_service::NotificationService;
use crate::domain::{model::error::Error, model::notification::Notification};

#[derive(Default)]
pub(super) struct NotificationSubscription;

#[Subscription]
impl NotificationSubscription {
    #[graphql_mac::require_header("session-token")]
    async fn notification_feed(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Pin<Box<dyn Stream<Item = FieldResult<Notification>> + Send>>> {
        let notification_service = ctx.data::<NotificationService>()?;
        let stream = notification_service
            .subscribe()
            .await
            .map(|value| Ok(value));

        Ok(Box::pin(stream))
    }
}
