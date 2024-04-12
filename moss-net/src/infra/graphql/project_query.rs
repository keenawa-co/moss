use async_graphql::{Context, Object};
use std::sync::Arc;

use crate::domain::{
    model::{
        project::{NewProjectInput, Project},
        RecordObject,
    },
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
    async fn delete_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> async_graphql::Result<RecordObject<i32>> {
        let project_service = ctx.data::<Arc<ProjectService>>()?;
        let result = project_service.delete_by_id(id).await?;

        Ok(result)
    }
}
