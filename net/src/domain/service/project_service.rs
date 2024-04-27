use common::thing::Thing;
use fs::real;
use sea_orm::DatabaseConnection;
use std::{path::PathBuf, sync::Arc};

use crate::{
    domain::{
        self,
        model::project::{NewProjectInput, Project},
        port::{ProjectRepository, ProjectSessionStorage},
    },
    infra::database::sqlite::ProjectClient,
};

#[derive(Debug, Clone)]
pub struct ProjectService {
    real_fs: Arc<real::FileSystem>,
    repo: Arc<dyn ProjectRepository>,
}

pub(crate) struct CreateProjectOutput {
    pub entity: Project,
    pub project_db: DatabaseConnection,
    pub cache_db: DatabaseConnection,
}

impl ProjectService {
    pub fn new(real_fs: Arc<real::FileSystem>, repo: Arc<dyn ProjectRepository>) -> Self {
        Self { real_fs, repo }
    }

    pub async fn create_project(
        &self,
        input: NewProjectInput,
    ) -> domain::Result<CreateProjectOutput> {
        let pwd_output =
            pwd::init::create_from_scratch(&PathBuf::from(&input.path), &self.real_fs).await?;

        Ok(CreateProjectOutput {
            entity: self.repo.create_project(input).await?,
            project_db: pwd_output.project_db,
            cache_db: pwd_output.cache_db,
        })
    }

    pub async fn delete_by_id(&self, id: String) -> domain::Result<Thing> {
        let result = self.repo.delete_by_id(id).await?;

        Ok(result)
    }
}

pub struct ProjectSessionService {
    storage: Arc<dyn ProjectSessionStorage>,
    // cache: Option<Arc<dyn ProjectCacheStorage>>,
}

impl ProjectSessionService {
    pub fn new(client: ProjectClient) -> Self {
        Self {
            storage: Arc::new(client),
        }
    }
}
