mod migration;

mod ignore_list_repo_impl;
mod project_meta_repo_impl;
mod session_repo_impl;

pub(crate) use migration::ProjectMigrator;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use self::{
    ignore_list_repo_impl::IgnoreListRepositoryImpl,
    project_meta_repo_impl::ProjectMetaRepositoryImpl, session_repo_impl::SessionRepositoryImpl,
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

    // pub(crate) fn session_repo(&self) -> &impl SessionRepository {
    //     &self.session_repo
    // }
}

#[derive(Debug)]
pub struct ProjectDatabaseClient {
    watch_list_repo: Arc<IgnoreListRepositoryImpl>,
}

impl ProjectDatabaseClient {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            watch_list_repo: Arc::new(IgnoreListRepositoryImpl::new(conn)),
        }
    }

    pub fn watch_list_repo(&self) -> Arc<IgnoreListRepositoryImpl> {
        self.watch_list_repo.clone()
    }
}
