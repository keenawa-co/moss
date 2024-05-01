use async_graphql::{Context, Object, Result as GraphqlResult};
use chrono::{Duration, Utc};
use tokio::sync::RwLock;

use crate::domain::{
    model::session::{CreateSessionInput, Session},
    service::{ProjectService, SessionService},
};

#[derive(Default)]
pub(super) struct SessionMutation;

#[Object]
impl SessionMutation {
    async fn create_session(
        &self,
        ctx: &Context<'_>,
        input: CreateSessionInput,
    ) -> GraphqlResult<Session> {
        let session_service = ctx.data::<RwLock<SessionService>>()?;
        let session_project_service = ctx.data::<RwLock<Option<ProjectService>>>()?;

        let session_service_lock = session_service.write().await;
        let session = session_service_lock
            .create_session(&input, session_project_service)
            .await?;

        Ok(session)
    }

    #[graphql(name = "getRecentSessionList")]
    async fn get_recent(
        &self,
        ctx: &Context<'_>,
        #[graphql(default_with = "(Utc::now() - Duration::days(30)).timestamp()")] start_time: i64,
        #[graphql(validator(minimum = 1, maximum = 10), default = 10)] limit: u64,
    ) -> GraphqlResult<Vec<Session>> {
        let session_service_lock = ctx.data::<RwLock<SessionService>>()?.write().await;
        let result = session_service_lock
            .get_recent_list(start_time, limit)
            .await?;

        Ok(result)
    }
}
