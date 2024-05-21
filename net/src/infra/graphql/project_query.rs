use async_graphql::{Context, FieldResult, Object, Result as GraphqlResult, Subscription};
use futures::{Stream, StreamExt};
use graphql_utl::{path::Path as PathGraphQL, GraphQLExtendError};
use http::HeaderMap;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use types::{id::NanoId, thing::Thing};

use crate::domain::{
    model::{
        error::Error,
        notification::Notification,
        project::{CreateProjectInput, ProjectMeta},
        session::SessionTokenClaims,
    },
    service::{
        notification_service::NotificationService, project_meta_service::ProjectMetaService,
        project_service::ProjectService,
    },
};

pub(super) struct ProjectMutation {
    pub project_meta_service: Arc<ProjectMetaService>,
    pub project_service: Arc<RwLock<ProjectService>>,
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
    async fn delete_by_id(&self, _ctx: &Context<'_>, id: NanoId) -> GraphqlResult<Thing<NanoId>> {
        Ok(self
            .project_meta_service
            .delete_project_by_id(&id)
            .await
            .extend_error()?)
    }

    #[graphql(name = "appendToProjectExclude")]
    #[graphql_mac::require_header("session-token")]
    async fn append_to_exclude_list(
        &self,
        ctx: &Context<'_>,
        input_list: Vec<PathGraphQL>,
    ) -> GraphqlResult<Vec<String>> {
        let sess_claims = ctx.data::<SessionTokenClaims>()?;
        let project_service_lock = self.project_service.write().await;

        let result = project_service_lock
            .append_to_monitoring_exclude_list(
                &input_list.iter().map(Into::into).collect::<Vec<PathBuf>>(),
            )
            .await
            .extend_error()?;

        for item in input_list.iter() {
            self.notification_service
                .send(Notification::create_client(format!(
                    "Path {item} has been successfully added to the ignore list"
                )))
                .await?;
        }

        Ok(result)
    }

    #[graphql(name = "removeFromProjectExclude")]
    #[graphql_mac::require_header("session-token")]
    async fn remove_from_exclude_list(
        &self,
        ctx: &Context<'_>,
        input_list: Vec<PathGraphQL>,
    ) -> GraphqlResult<Vec<String>> {
        let sess_claims = ctx.data::<SessionTokenClaims>()?;
        let project_service_lock = self.project_service.write().await;

        let result = project_service_lock
            .remove_from_monitoring_exclude_list(
                &input_list
                    .into_iter()
                    .map(|path| path.into())
                    .collect::<Vec<PathBuf>>(),
            )
            .await?;

        for item in &result {
            self.notification_service
                .send(Notification::create_client(format!(
                    "Path {item} has been successfully added to the ignore list"
                )))
                .await?;
        }

        Ok(result)
    }
}

pub(super) struct ProjectSubscription {
    pub project_service: Arc<RwLock<ProjectService>>,
}

#[Subscription]
impl ProjectSubscription {
    async fn explorer_event_feed(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<serde_json::Value>>> {
        // dbg!(ctx.http_header_contains("session-token"));
        // let sess_claims = ctx.data::<HeaderMap>()?;

        let project_service_lock = self.project_service.read().await;
        let stream = project_service_lock.explorer_event_feed().await?;

        Ok(stream.map(|event| {
            Ok(serde_json::json!({
                "event": event.kind,
                "path":  event.entry.path.to_path_buf().to_string_lossy().to_string()
            }))
        }))
    }
}
