use async_graphql::{Context, Object, Result as GraphqlResult};
use chrono::{Duration, Utc};
use graphql_utl::path::Path as PathGraphQL;
use graphql_utl::GraphQLExtendError;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use types::id::NanoId;

use crate::domain::{
    model::{
        session::{Session, SessionEntity, SessionToken},
        OptionExtension,
    },
    service::{
        project_service::ProjectService,
        session_service::SessionService,
        workspace_service::{CreateConfig, WorkspaceService},
    },
};

pub(super) struct SessionMutation<'a> {
    pub session_service: Arc<SessionService>,
    pub project_service: Arc<ProjectService<'a>>,
    pub workspace_service: Arc<WorkspaceService>,
}

#[Object]
impl<'a> SessionMutation<'a> {
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
        let project_path = PathBuf::from(&session_entity.project_meta.as_ref().unwrap().source);

        self.workspace_service
            .create(&CreateConfig {
                project_path: &project_path,
            })
            .await?;

        {
            let settings_file = self
                .workspace_service
                .get_settings()
                .ok_or_resource_precondition_required(
                    "Session must be initialized first, settings.json file is not defined",
                    None,
                )?;

            // let project_service_lock = self.project_service.write().await;
            self.project_service
                .start_project(&project_path, settings_file)
                .await?;
        }

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
        let project_path = PathBuf::from(&session_entity.project_meta.as_ref().unwrap().source);

        self.workspace_service
            .create(&CreateConfig {
                project_path: &project_path,
            })
            .await?;

        {
            let settings_file = self
                .workspace_service
                .get_settings()
                .ok_or_resource_precondition_required(
                    "Session must be initialized first, settings.json file is not defined",
                    None,
                )?;

            self.project_service
                .start_project(&project_path, settings_file)
                .await?;
        }

        Ok(Session {
            id: session_entity.id,
            token: session_token,
            project_meta: session_entity.project_meta,
            created_at: session_entity.created_at,
        })
    }
}

pub(crate) struct SessionQuery {
    pub session_service: Arc<SessionService>,
}

#[Object]
impl SessionQuery {
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
