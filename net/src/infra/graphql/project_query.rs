use async_graphql::{Context, Object, Result as GraphqlResult};
use common::{id::NanoId, thing::Thing};
use graphql_utl::{path::Path as PathGraphQL, GraphQLExtendError};
use std::path::PathBuf;
use tokio::sync::RwLock;

use crate::domain::{
    model::{
        error::Error,
        notification::Notification,
        project::{CreateProjectInput, IgnoredSource, ProjectMeta},
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
            .delete_project_by_id(&id)
            .await
            .extend_error()?)
    }

    #[graphql(name = "appendToProjectIgnored")]
    // #[graphql_mac::check_header("session-id")]
    async fn append_to_ignore_list(
        &self,
        ctx: &Context<'_>,
        input_list: Vec<PathGraphQL>,
    ) -> GraphqlResult<Vec<IgnoredSource>> {
        let notification_service = ctx.data::<NotificationService>()?;
        let session_service_lock = ctx.data::<RwLock<SessionService>>()?.write().await;
        let project_service_lock = ctx.data::<RwLock<Option<ProjectService>>>()?.write().await;
        let project_service = project_service_lock
            .as_ref()
            .ok_or_resource_precondition_required("Session must be initialized first", None)
            .extend_error()?;

        let result_list = project_service
            .append_to_ignore_list(&input_list.iter().map(Into::into).collect::<Vec<PathBuf>>())
            .await
            .extend_error()?;

        let (session_id, project_id) = {
            // Panic here should never happen.
            // Always indicates a bug in a SessionService, since at the time
            // of the call all data must already be in the context.
            //
            // Possible causes of panic:
            // - Lack of data in the context
            // - Data stored in another type Value

            let _session_id = session_service_lock
                .get_from_context("session_id")
                .and_then(|v| v.as_str())
                .and_then(|v| Some(NanoId::from(v)))
                .unwrap();

            let _project_id = session_service_lock
                .get_from_context("project_id")
                .and_then(|v| v.as_str())
                .and_then(|v| Some(NanoId::from(v)))
                .unwrap();

            (_session_id, _project_id)
        };

        for item in input_list.iter() {
            notification_service
                .send(Notification {
                    id: NanoId::new(),
                    project_id: project_id.clone(),
                    session_id: session_id.clone(),
                    summary: format!("Path {item} has been successfully added to the ignore list"),
                })
                .await?;
        }

        Ok(result_list)
    }

    #[graphql(name = "removeFromProjectIgnored")]
    async fn remove_from_ignore_list(&self, ctx: &Context<'_>, id: NanoId) -> GraphqlResult<Thing> {
        let notification_service = ctx.data::<NotificationService>()?;
        let session_service_lock = ctx.data::<RwLock<SessionService>>()?.write().await;
        let project_service_lock = ctx.data::<RwLock<Option<ProjectService>>>()?.write().await;
        let project_service = project_service_lock
            .as_ref()
            .ok_or_resource_precondition_required("Session must be initialized first", None)
            .extend_error()?;

        let result = project_service.remove_from_ignore_list(&id).await?;

        let (session_id, project_id) = {
            // Panic here should never happen.
            // Always indicates a bug in a SessionService, since at the time
            // of the call all data must already be in the context.
            //
            // Possible causes of panic:
            // - Lack of data in the context
            // - Data stored in another type Value

            let _session_id = session_service_lock
                .get_from_context("session_id")
                .and_then(|v| v.as_str())
                .and_then(|v| Some(NanoId::from(v)))
                .unwrap();

            let _project_id = session_service_lock
                .get_from_context("project_id")
                .and_then(|v| v.as_str())
                .and_then(|v| Some(NanoId::from(v)))
                .unwrap();

            (_session_id, _project_id)
        };

        notification_service
            .send(Notification {
                id: NanoId::new(),
                project_id: project_id,
                session_id: session_id,
                summary: format!("Path {id} has been successfully added to the ignore list"),
            })
            .await?;

        Ok(result)
    }
}
