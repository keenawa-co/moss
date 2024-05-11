use async_graphql::{Context, Object, Result as GraphqlResult};
use common::{id::NanoId, thing::Thing};
use graphql_utl::{path::Path as PathGraphQL, GraphQLExtendError};
use http::HeaderMap;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::{
    model::{
        error::Error,
        notification::Notification,
        project::{CreateProjectInput, IgnoredSource, ProjectMeta},
        session::SessionTokenClaims,
        OptionExtension,
    },
    service::{
        notification_service::NotificationService, project_meta_service::ProjectMetaService,
        project_service::ProjectService,
    },
};

pub(super) struct ProjectMutation {
    pub project_meta_service: Arc<ProjectMetaService>,
    pub project_service: Arc<RwLock<Option<ProjectService>>>,
    pub notification_service: Arc<NotificationService>,
}

#[Object]
impl ProjectMutation {
    #[graphql(name = "createProject")]
    async fn create_project(
        &self,
        _ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> GraphqlResult<ProjectMeta> {
        Ok(self.project_meta_service.create_project(&input).await?)
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, _ctx: &Context<'_>, id: NanoId) -> GraphqlResult<Thing> {
        Ok(self
            .project_meta_service
            .delete_project_by_id(&id)
            .await
            .extend_error()?)
    }

    #[graphql(name = "appendToProjectIgnored")]
    #[graphql_mac::require_header("session-token")]
    async fn append_to_ignore_list(
        &self,
        ctx: &Context<'_>,
        input_list: Vec<PathGraphQL>,
    ) -> GraphqlResult<Vec<IgnoredSource>> {
        let sess_claims = ctx.data::<SessionTokenClaims>()?;
        let project_service_lock = self.project_service.write().await;
        let project_service = project_service_lock
            .as_ref()
            .ok_or_resource_precondition_required("Session must be initialized first", None)
            .extend_error()?;

        let result_list = project_service
            .append_to_ignore_list(&input_list.iter().map(Into::into).collect::<Vec<PathBuf>>())
            .await
            .extend_error()?;

        for item in input_list.iter() {
            self.notification_service
                .send(Notification {
                    id: NanoId::new(),
                    project_id: sess_claims.project_id.clone(),
                    session_id: sess_claims.session_id.clone(),
                    summary: format!("Path {item} has been successfully added to the ignore list"),
                })
                .await?;
        }

        Ok(result_list)
    }

    #[graphql(name = "removeFromProjectIgnored")]
    #[graphql_mac::require_header("session-token")]
    async fn remove_from_ignore_list(&self, ctx: &Context<'_>, id: NanoId) -> GraphqlResult<Thing> {
        let sess_claims = ctx.data::<SessionTokenClaims>()?;
        let project_service_lock = self.project_service.write().await;
        let project_service = project_service_lock
            .as_ref()
            .ok_or_resource_precondition_required("Session must be initialized first", None)
            .extend_error()?;

        let result = project_service.remove_from_ignore_list(&id).await?;

        self.notification_service
            .send(Notification {
                id: NanoId::new(),
                project_id: sess_claims.project_id.clone(),
                session_id: sess_claims.session_id.clone(),
                summary: format!("Path {id} has been successfully added to the ignore list"),
            })
            .await?;

        Ok(result)
    }
}
