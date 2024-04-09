mod portal_repo;
mod project_repo;

use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use self::{portal_repo::PortalRepositoryImpl, project_repo::ProjectRepositoryImpl};
use crate::domain;

const PROJECT_TABLE_NAME: &str = "projects";

pub(crate) struct SurrealOnDisk {
    portal_repo: Arc<PortalRepositoryImpl>,
    project_repo: Arc<ProjectRepositoryImpl>,
}

impl SurrealOnDisk {
    pub(crate) fn new(client: Arc<Surreal<Client>>) -> domain::Result<Self> {
        let db = Self {
            portal_repo: Arc::new(PortalRepositoryImpl::new(client.clone())),
            project_repo: Arc::new(ProjectRepositoryImpl::new(client.clone())),
        };

        Ok(db)
    }

    pub fn portal_repo(&self) -> Arc<PortalRepositoryImpl> {
        self.portal_repo.clone()
    }

    pub fn project_repo(&self) -> Arc<ProjectRepositoryImpl> {
        self.project_repo.clone()
    }
}
