mod portal_repo;

use serde::Deserialize;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use self::portal_repo::PortalRepositoryImpl;

#[derive(Debug, Clone, Deserialize)]
pub struct TableSet {
    pub portal_recent: String,
}

pub struct SurrealOnDisk {
    portal_repo: Arc<PortalRepositoryImpl>,
}

impl SurrealOnDisk {
    pub fn new(client: Arc<Surreal<Client>>, tables: &TableSet) -> Self {
        Self {
            portal_repo: Arc::new(PortalRepositoryImpl::new(client, &tables.portal_recent)),
        }
    }

    pub fn portal_repo(&self) -> Arc<PortalRepositoryImpl> {
        self.portal_repo.clone()
    }
}
