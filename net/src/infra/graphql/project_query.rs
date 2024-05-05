use async_graphql::{Context, Object, Result as GraphqlResult};
use common::{id::NanoId, thing::Thing};
use gqlutl::{path::Path as PathGraphQL, GraphQLExtendError};
use std::{fmt::format, path::PathBuf};
use tokio::sync::RwLock;

use crate::domain::{
    model::{
        notification::Notification,
        project::{CreateProjectInput, ProjectMeta},
        OptionExtension,
    },
    service::{
        notification_service::NotificationService, project_meta_service::ProjectMetaService,
        project_service::ProjectService, session_service::SessionService,
    },
};

#[derive(Default)]
pub(super) struct ProjectMutation;

#[Object]
impl ProjectMutation {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> GraphqlResult<ProjectMeta> {
        let project_meta_service = ctx.data::<ProjectMetaService>()?;

        Ok(project_meta_service.create_project(&input).await?)
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, ctx: &Context<'_>, id: NanoId) -> GraphqlResult<Thing> {
        let project_meta_service = ctx.data::<ProjectMetaService>()?;

        Ok(project_meta_service
            .delete_project_by_id(id)
            .await
            .extend_error()?)
    }

    #[graphql(name = "createProjectIgnoreList")]
    async fn create_ignore_list(
        &self,
        ctx: &Context<'_>,
        input_list: Vec<PathGraphQL>,
    ) -> GraphqlResult<Thing> {
        let notification_service = ctx.data::<NotificationService>()?;
        let session_service_lock = ctx.data::<RwLock<SessionService>>()?.write().await;
        let project_service_lock = ctx.data::<RwLock<Option<ProjectService>>>()?.write().await;
        let project_service = project_service_lock
            .as_ref()
            .ok_or_resource_precondition_required("Session must be initialized first", None)
            .extend_error()?;

        project_service
            .create_ignore_list(&input_list.iter().map(Into::into).collect::<Vec<PathBuf>>())
            .await
            .extend_error()?;

        for item in input_list {
            notification_service
                .send(Notification {
                    id: NanoId::new(),
                    project_id: session_service_lock.project_id().clone().unwrap(),
                    session_id: session_service_lock.session_id().clone().unwrap(),
                    summary: format!("Path {item} has been successfully added to the ignore list"),
                })
                .await?;
        }

        Ok(Thing {
            id: "test".to_string(),
        })
    }
}
