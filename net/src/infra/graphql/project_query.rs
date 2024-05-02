use async_graphql::{Context, Error as GraphQLError, Object, Result as GraphqlResult};
use common::{id::NanoId, thing::Thing};
use gqlutl::{path::Path as PathGraphQL, GraphQLExtendError};
use std::path::PathBuf;
use tokio::sync::RwLock;

use crate::domain::{
    model::project::{CreateProjectInput, ProjectMeta},
    service::{project_meta_service::ProjectMetaService, project_service::ProjectService},
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

    #[graphql(name = "getProjectListByIds")]
    async fn get_list_by_ids(
        &self,
        ctx: &Context<'_>,
        ids: Vec<NanoId>,
    ) -> GraphqlResult<Vec<ProjectMeta>> {
        let project_meta_service = ctx.data::<ProjectMetaService>()?;

        Ok(project_meta_service
            .get_project_list_by_ids(&ids)
            .await
            .extend_error()?)
    }

    // #[graphql(name = "createProjectIgnoreList")]
    // async fn create_ignore_list(
    //     &self,
    //     ctx: &Context<'_>,
    //     input_list: Vec<PathGraphQL>,
    // ) -> GraphqlResult<Thing> {
    //     let project_service = ctx.data::<RwLock<Option<ProjectService>>>()?;
    //     let project_service_lock = {
    //         let service_lock = project_service.write().await;
    //         service_lock
    //             .as_ref()
    //             .ok_or_else(|| GraphQLError::from("ProjectService is not initialized"))
    //     }?;

    //     // let project_service = ctx.data::<RwLock<Option<ProjectService>>>()?;
    //     // let project_service_lock = project_service.write().await;

    //     // let service = project_service_lock
    //     //     .as_ref()
    //     //     .ok_or_else(|| GraphQLError::from("ProjectService is not initialized"))?;

    //     project_service_lock
    //         .create_ignore_list(
    //             &input_list
    //                 .iter()
    //                 .map(|item| item.into())
    //                 .collect::<Vec<PathBuf>>(),
    //         )
    //         .await?;

    //     Ok(Thing {
    //         id: "test".to_string(),
    //     })
    // }
}
