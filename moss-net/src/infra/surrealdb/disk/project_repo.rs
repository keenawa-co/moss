use axum::async_trait;
use chrono::Utc;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::domain::{
    self,
    model::project::{NewProjectInput, Project},
};

#[derive(Debug)]
pub(crate) struct ProjectRepositoryImpl {
    client: Arc<Surreal<Client>>,
}

impl ProjectRepositoryImpl {
    pub(super) fn new(client: Arc<Surreal<Client>>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl domain::port::ProjectRepository for ProjectRepositoryImpl {
    async fn create_project(&self, input: NewProjectInput) -> domain::Result<Vec<Project>> {
        let result: Vec<Project> = self
            .client
            .create(super::PROJECT_TABLE_NAME)
            .content(Project {
                id: None,
                path: input.path,
                updated: Utc::now().timestamp(),
            })
            .await?;

        Ok(result)
    }

    async fn delete_by_id(&self, id: String) -> domain::Result<Option<Project>> {
        let result: Option<Project> = self.client.delete((super::PROJECT_TABLE_NAME, id)).await?;

        Ok(result)
    }
}