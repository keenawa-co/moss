use async_graphql::{Context, Object};
use common::thing::Thing;

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
    ) -> async_graphql::Result<Project> {
        let project_service = ctx.data::<ProjectService>()?;
        let project_entity = project_service.create_project(&input).await?;

        Ok(project_entity)
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Thing> {
        let project_service = ctx.data::<ProjectService>()?;
        let result: Thing = project_service.delete_by_id(id).await?;

        Ok(result)
    }
}
