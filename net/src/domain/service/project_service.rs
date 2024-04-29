use common::thing::Thing;
use fs::real;
use std::sync::Arc;

use crate::domain::{
    self,
    model::project::{CreateProjectInput, Project},
    port::ProjectRepository,
};

#[derive(Debug, Clone)]
pub struct ProjectService {
    realfs: Arc<real::FileSystem>,
    project_repo: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(realfs: Arc<real::FileSystem>, repo: Arc<dyn ProjectRepository>) -> Self {
        Self {
            realfs,
            project_repo: repo,
        }
    }

    pub async fn create_project(&self, input: &CreateProjectInput) -> domain::Result<Project> {
        let project_entity = self.project_repo.create_project(&input).await?;

        Ok(project_entity)
    }

    pub async fn delete_by_id(&self, id: String) -> domain::Result<Thing> {
        let result = self.project_repo.delete_by_id(id).await?;

        Ok(result)
    }
}
