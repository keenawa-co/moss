use async_graphql::{Context, ErrorExtensions, Object, Result as GraphqlResult, ResultExt};
use common::{id::NanoId, thing::Thing};
use gqlutl::GraphQLExtendError;

use crate::domain::{
    model::project::{CreateProjectInput, ProjectMeta},
    service::ProjectMetaService,
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
}
