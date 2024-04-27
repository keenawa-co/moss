mod project_repo_impl;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::domain::port::ProjectSessionStorage;

use self::project_repo_impl::ProjectRepositoryImpl;

pub struct RootClient {
    project_repo: Arc<ProjectRepositoryImpl>,
}

impl RootClient {
    pub(crate) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            project_repo: Arc::new(ProjectRepositoryImpl::new(conn)),
        }
    }

    pub(crate) fn project_repo(&self) -> Arc<ProjectRepositoryImpl> {
        self.project_repo.clone()
    }
}

#[derive(Debug)]
pub struct ProjectClient {}

impl ProjectClient {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {}
    }
}

#[async_trait]
impl ProjectSessionStorage for ProjectClient {}
