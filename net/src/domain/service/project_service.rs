use common::thing::Thing;
use fs::real;
use std::{path::Path, sync::Arc};

use crate::domain::{
    self,
    model::project::{NewProjectInput, Project},
    port::ProjectRepository,
};

#[derive(Debug, Clone)]
pub struct ProjectService {
    real_fs: Arc<real::FileSystem>,
    repo: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(real_fs: Arc<real::FileSystem>, repo: Arc<dyn ProjectRepository>) -> Self {
        Self { real_fs, repo }
    }

    pub async fn create_project(&self, input: NewProjectInput) -> domain::Result<Project> {
        initwd::create_from_scratch(&input.path, &self.real_fs).await?;
        let result = self.repo.create_project(input).await?;

        Ok(result)
    }

    pub async fn delete_by_id(&self, id: String) -> domain::Result<Thing> {
        let result = self.repo.delete_by_id(id).await?;

        Ok(result)
    }
}
