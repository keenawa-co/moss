mod migrate;
mod migration;

mod project_repo_impl;
mod watch_list_repo_impl;

pub(crate) use migration::ProjectMigrator;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use self::{
    project_repo_impl::ProjectRepositoryImpl, watch_list_repo_impl::WatchListRepositoryImpl,
};

pub struct RootDatabaseClient {
    project_repo: Arc<ProjectRepositoryImpl>,
}

impl RootDatabaseClient {
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
pub struct ProjectDatabaseClient {
    watch_list_repo: Arc<WatchListRepositoryImpl>,
}

impl ProjectDatabaseClient {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            watch_list_repo: Arc::new(WatchListRepositoryImpl::new(conn)),
        }
    }

    pub fn watch_list_repo(&self) -> Arc<WatchListRepositoryImpl> {
        self.watch_list_repo.clone()
    }
}
