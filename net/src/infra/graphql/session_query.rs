use async_graphql::{Context, Object};
use tokio::sync::RwLock;

use crate::domain::{
    model::session::{CreateSessionInput, CreateSessionOutput},
    service::{SessionProjectService, SessionService},
};

#[derive(Default)]
pub(super) struct SessionMutation;

#[Object]
impl SessionMutation {
    async fn create_session(
        &self,
        ctx: &Context<'_>,
        input: CreateSessionInput,
    ) -> async_graphql::Result<CreateSessionOutput> {
        let session_service = ctx.data::<RwLock<SessionService>>()?;
        let session_project_service = ctx.data::<RwLock<Option<SessionProjectService>>>()?;

        let session_service_lock = session_service.write().await;
        let session = session_service_lock
            .create_session(input.project_id, session_project_service)
            .await?;

        Ok(session.into())
    }
}
