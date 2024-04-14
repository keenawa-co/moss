use moss_core::model::thing::Thing;
use std::sync::Arc;

use crate::domain::{
    self,
    model::project::{NewProjectInput, Project},
    port::ProjectRepository,
};

#[derive(Debug, Clone)]
pub struct ProjectService {
    pub repo: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_project(&self, input: NewProjectInput) -> domain::Result<Project> {
        let result = self.repo.create_project(input).await?;

        Ok(result)
    }

    pub async fn delete_by_id(&self, id: String) -> domain::Result<Thing> {
        let result = self.repo.delete_by_id(id).await?;

        Ok(result)
    }
}
