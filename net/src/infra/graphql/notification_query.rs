use async_graphql::{Context, FieldResult, Subscription};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};

use crate::domain::{
    model::notification::Notification, service::notification_service::NotificationService,
};

#[derive(Default)]
pub(super) struct NotificationSubscription;

#[Subscription]
impl NotificationSubscription {
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
