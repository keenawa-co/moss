mod project_repo;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use self::project_repo::ProjectRepositoryImpl;

pub struct SQLiteClient {
    project_repo: Arc<ProjectRepositoryImpl>,
}

impl SQLiteClient {
    pub(crate) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            project_repo: Arc::new(ProjectRepositoryImpl::new(conn)),
        }
    }

    pub(crate) fn project_repo(&self) -> Arc<ProjectRepositoryImpl> {
        self.project_repo.clone()
    }
}
