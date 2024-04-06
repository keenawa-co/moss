mod portal_repo;

use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use self::portal_repo::PortalRepositoryImpl;

pub struct SurrealOnDisk {
    portal_repo: Arc<PortalRepositoryImpl>,
}

impl SurrealOnDisk {
    pub fn new(client: Arc<Surreal<Client>>) -> Self {
        Self {
            portal_repo: Arc::new(PortalRepositoryImpl::new(client)),
        }
    }

    pub fn portal_repo(&self) -> Arc<PortalRepositoryImpl> {
        self.portal_repo.clone()
    }
}
