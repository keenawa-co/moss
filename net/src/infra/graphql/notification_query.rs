use async_graphql::{Context, FieldResult, Subscription};
use graphql_utl::GraphQLExtendError;
use http::HeaderMap;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::{Stream, StreamExt};

use crate::domain::service::notification_service::NotificationService;
use crate::domain::{model::error::Error, model::notification::Notification};

pub(super) struct NotificationSubscription {
    pub notification_service: Arc<NotificationService>,
}

#[Subscription]
impl NotificationSubscription {
    // #[graphql_mac::require_header("session-token")]
    async fn notification_feed(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Pin<Box<dyn Stream<Item = FieldResult<Value>> + Send>>> {
        let stream = self
            .notification_service
            .subscribe()
            .await
            .map(|value| Ok(serde_json::to_value(value)?));

        Ok(Box::pin(stream))
    }
}
