use async_graphql::{Context, Object, Result as GraphqlResult};
use common::{id::NanoId, thing::Thing};

use crate::domain::{
    model::project::{CreateProjectInput, Project},
    service::ProjectService,
};

#[derive(Default)]
pub(super) struct ProjectMutation;

#[Object]
impl ProjectMutation {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> GraphqlResult<Project> {
        let project_service = ctx.data::<ProjectService>()?;

        Ok(project_service.create_project(&input).await?)
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, ctx: &Context<'_>, id: NanoId) -> GraphqlResult<Thing> {
        let project_service = ctx.data::<ProjectService>()?;
        Ok(project_service.delete_project_by_id(id).await?)
    }

    #[graphql(name = "getProjectListByIds")]
    async fn get_list_by_ids(
        &self,
        ctx: &Context<'_>,
        ids: Vec<NanoId>,
    ) -> GraphqlResult<Vec<Project>> {
        let project_service = ctx.data::<ProjectService>()?;
        Ok(project_service.get_project_list_by_ids(&ids).await?)
    }
}
