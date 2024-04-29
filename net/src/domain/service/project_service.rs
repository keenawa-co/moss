use common::{id::MNID, thing::Thing};
use std::sync::Arc;

use crate::domain::{
    self,
    model::project::{CreateProjectInput, Project},
    port::ProjectRepository,
};

#[derive(Debug, Clone)]
pub struct ProjectService {
    project_repo: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { project_repo: repo }
    }

    pub async fn create_project(&self, input: &CreateProjectInput) -> domain::Result<Project> {
        let project_entity = self.project_repo.create(&input).await?;

        Ok(project_entity)
    }

    pub async fn delete_project_by_id(&self, id: MNID) -> domain::Result<Thing> {
        let result = self.project_repo.delete_by_id(id).await?;

        Ok(result)
    }

    pub async fn get_project_list_by_ids(&self, ids: &Vec<MNID>) -> domain::Result<Vec<Project>> {
        Ok(self.project_repo.get_list_by_ids(ids).await?)
    }
}
