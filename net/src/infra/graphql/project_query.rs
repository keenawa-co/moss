use async_graphql::{Context, Object};
use common::thing::Thing;
use std::sync::Arc;

use crate::domain::{
    model::project::{NewProjectInput, Project},
    service::ProjectService,
};

#[derive(Default)]
pub(super) struct ProjectMutation;

#[Object]
impl ProjectMutation {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: NewProjectInput,
    ) -> async_graphql::Result<Project> {
        let project_service = ctx.data::<Arc<ProjectService>>()?;
        let result = project_service.create_project(input).await?;

        Ok(result)
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Thing> {
        let project_service = ctx.data::<Arc<ProjectService>>()?;
        let result = project_service.delete_by_id(id).await?;

        Ok(result)
    }
}
