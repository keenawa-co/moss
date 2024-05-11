use std::{path::PathBuf, sync::Arc};

use async_graphql::{Context, Object, Result as GraphqlResult};
use chrono::{Duration, Utc};
use common::id::NanoId;
use graphql_utl::path::Path as PathGraphQL;
use graphql_utl::GraphQLExtendError;
use tokio::sync::RwLock;

use crate::domain::{
    model::session::{Session, SessionEntity, SessionToken},
    service::{project_service::ProjectService, session_service::SessionService},
};

pub(super) struct SessionMutation {
    pub session_service: Arc<SessionService>,
    pub project_service: Arc<RwLock<Option<ProjectService>>>,
}

#[Object]
impl SessionMutation {
    async fn create_session(
        &self,
        _ctx: &Context<'_>,
        project_source: PathGraphQL,
    ) -> GraphqlResult<Session> {
        let session_entity = self
            .session_service
            .create_session(&project_source.into())
            .await
            .extend_error()?;
        let session_token = SessionToken::try_from(session_entity.clone())?;

        let mut project_service_lock = self.project_service.write().await;
        let project_path = PathBuf::from(&session_entity.project_meta.as_ref().unwrap().source);
        *project_service_lock = Some(ProjectService::new(&project_path).await?);

        Ok(Session {
            id: session_entity.id,
            token: session_token,
            project_meta: session_entity.project_meta,
            created_at: session_entity.created_at,
        })
    }

    async fn restore_session(
        &self,
        _ctx: &Context<'_>,
        session_id: NanoId,
    ) -> GraphqlResult<Session> {
        let session_entity = self
            .session_service
            .restore_session(session_id)
            .await
            .extend_error()?;
        let session_token = SessionToken::try_from(session_entity.clone())?;

        let mut project_service_lock = self.project_service.write().await;
        let project_path = PathBuf::from(&session_entity.project_meta.as_ref().unwrap().source);
        *project_service_lock = Some(ProjectService::new(&project_path).await?);

        Ok(Session {
            id: session_entity.id,
            token: session_token,
            project_meta: session_entity.project_meta,
            created_at: session_entity.created_at,
        })
    }

    // TODO: move to Query
    #[graphql(name = "getRecentSessions")]
    async fn get_recent(
        &self,
        _ctx: &Context<'_>,
        #[graphql(default_with = "(Utc::now() - Duration::days(30)).timestamp()")] start_time: i64,
        #[graphql(validator(minimum = 1, maximum = 10), default = 10)] limit: u64,
    ) -> GraphqlResult<Vec<SessionEntity>> {
        let result = self
            .session_service
            .get_recent_list(start_time, limit)
            .await
            .extend_error()?;

        Ok(result)
    }
}
