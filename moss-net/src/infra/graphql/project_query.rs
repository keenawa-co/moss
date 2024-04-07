#[derive(Default)]
pub struct ProjectMutation;

use std::sync::Arc;

use async_graphql::{Context, Object};

use crate::domain::{
    model::project::{NewProjectInput, Project},
    service::ProjectService,
};

#[Object]
impl ProjectMutation {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: NewProjectInput,
    ) -> async_graphql::Result<Vec<Project>> {
        let project_service = ctx.data::<Arc<ProjectService>>()?;
        let result = project_service.create_project(input).await?;

        Ok(result)
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Project> {
        let project_service = ctx.data::<Arc<ProjectService>>()?;
        let result = project_service.delete_by_id(id).await?;

        Ok(result)
    }
}
