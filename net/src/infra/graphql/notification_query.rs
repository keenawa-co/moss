use async_graphql::{Context, FieldResult, Subscription};
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::{Stream, StreamExt};

use crate::domain::service::notification_service::NotificationService;

pub(super) struct NotificationSubscription {
    pub notification_service: Arc<NotificationService>,
}

#[Subscription]
impl NotificationSubscription {
    // #[graphql_mac::require_header("session-token")]
    async fn notification_feed(
        &self,
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<Pin<Box<dyn Stream<Item = FieldResult<Value>> + Send>>> {
        let stream = self
            .notification_service
            .subscribe()
            .await
            .map(|value| Ok(serde_json::to_value(value)?));

        Ok(Box::pin(stream))
    }
}
