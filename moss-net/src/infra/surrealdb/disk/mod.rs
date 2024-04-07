mod portal_repo;
mod project_repo;

use serde::Deserialize;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use self::{portal_repo::PortalRepositoryImpl, project_repo::ProjectRepositoryImpl};

#[derive(Debug, Clone, Deserialize)]
pub struct TableSet {
    pub project_table: String,
}

pub struct SurrealOnDisk {
    portal_repo: Arc<PortalRepositoryImpl>,
    project_repo: Arc<ProjectRepositoryImpl>,
}

impl SurrealOnDisk {
    pub fn new(client: Arc<Surreal<Client>>, tables: &TableSet) -> Self {
        Self {
            portal_repo: Arc::new(PortalRepositoryImpl::new(
                client.clone(),
                &tables.project_table,
            )),
            project_repo: Arc::new(ProjectRepositoryImpl::new(
                client.clone(),
                &tables.project_table,
            )),
        }
    }

    pub fn portal_repo(&self) -> Arc<PortalRepositoryImpl> {
        self.portal_repo.clone()
    }

    pub fn project_repo(&self) -> Arc<ProjectRepositoryImpl> {
        self.project_repo.clone()
    }
}
