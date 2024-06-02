use app::event::PlatformEvent;
use async_graphql::{Context, FieldResult, Object, Result as GraphqlResult, Subscription};
use futures::{Stream, StreamExt};
use graphql_utl::{path::Path as PathGraphQL, GraphQLExtendError};
use http::HeaderMap;
use std::{path::PathBuf, sync::Arc};
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

pub(super) struct ProjectMutation<'a> {
    pub project_meta_service: Arc<ProjectMetaService>,
    pub project_service: Arc<ProjectService<'a>>,
    pub notification_service: Arc<NotificationService>,
}

#[Object]
impl<'a> ProjectMutation<'a> {
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

        let result = self
            .project_service
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

        let result = self
            .project_service
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

pub(super) struct ProjectSubscription<'a> {
    pub project_service: Arc<ProjectService<'a>>,
}

#[Subscription]
impl<'a> ProjectSubscription<'a> {
    async fn explorer_event_feed(
        &self,
        _ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = FieldResult<PlatformEvent>>> {
        let stream = self.project_service.event_live_stream().await?;

        Ok(stream.map(|event| Ok(event)))
    }
}
