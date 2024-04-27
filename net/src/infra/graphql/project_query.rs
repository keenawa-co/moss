use crate::{
    domain::{port::ProjectSessionStorage, service::ProjectSessionService},
    infra::database::sqlite::ProjectClient,
    internal,
};
use async_graphql::{Context, Object};
use common::thing::Thing;
use std::sync::{Arc, RwLock};

use crate::domain::{
    self,
    model::project::{NewProjectInput, Project},
    service::{ContextService, ProjectService},
};

#[derive(Default)]
pub(super) struct ProjectMutation;

#[Object]
impl ProjectMutation {
    #[graphql(name = "createProject")]
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: NewProjectInput,
    ) -> async_graphql::Result<Project> {
        let project_service = ctx.data::<ProjectService>()?;
        let create_output = project_service.create_project(input).await?;

        let mut context_service_lock = ctx
            .data::<RwLock<ContextService>>()?
            .write()
            .map_err(|e| internal!(e.to_string()))?;
        let project_ss = ProjectSessionService::new(ProjectClient::new(create_output.project_db));

        context_service_lock.set_project(create_output.entity.id.clone(), project_ss)?;

        Ok(create_output.entity)
    }

    #[graphql(name = "openProject")]
    async fn open_project(&self, _ctx: &Context<'_>) -> async_graphql::Result<Project> {
        unimplemented!()
    }

    #[graphql(name = "deleteProjectById")]
    async fn delete_by_id(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Thing> {
        let project_service = ctx.data::<ProjectService>()?;
        let result: Thing = project_service.delete_by_id(id).await?;

        Ok(result)
    }
}
