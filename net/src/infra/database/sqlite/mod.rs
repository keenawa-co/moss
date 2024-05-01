mod migration;

mod ignore_repo_impl;
mod project_meta_repo_impl;
mod session_repo_impl;

pub(crate) use migration::ProjectMigrator;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use self::{
    ignore_repo_impl::WatchListRepositoryImpl, project_meta_repo_impl::ProjectMetaRepositoryImpl,
    session_repo_impl::SessionRepositoryImpl,
};

pub struct RootDatabaseClient {
    project_repo: Arc<ProjectMetaRepositoryImpl>,
    session_repo: Arc<SessionRepositoryImpl>,
}

impl RootDatabaseClient {
    pub(crate) fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            project_repo: Arc::new(ProjectMetaRepositoryImpl::new(conn.clone())),
            session_repo: Arc::new(SessionRepositoryImpl::new(conn.clone())),
        }
    }

    pub(crate) fn project_meta_repo(&self) -> Arc<ProjectMetaRepositoryImpl> {
        self.project_repo.clone()
    }

    pub(crate) fn session_repo(&self) -> Arc<SessionRepositoryImpl> {
        self.session_repo.clone()
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
