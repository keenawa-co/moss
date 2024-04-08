mod portal_repo;
mod project_repo;

use serde_json::Value;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};

use self::{portal_repo::PortalRepositoryImpl, project_repo::ProjectRepositoryImpl};
use crate::domain;

pub(crate) struct SurrealOnDisk {
    portal_repo: Arc<PortalRepositoryImpl>,
    project_repo: Arc<ProjectRepositoryImpl>,
}

impl SurrealOnDisk {
    pub(crate) fn new(
        client: Arc<Surreal<Client>>,
        tables_raw_set: &serde_json::Map<String, Value>,
    ) -> domain::Result<Self> {
        let project_table_name = extract_table_name(tables_raw_set, "project_table")?;

        let db = Self {
            portal_repo: Arc::new(PortalRepositoryImpl::new(
                client.clone(),
                project_table_name,
            )),
            project_repo: Arc::new(ProjectRepositoryImpl::new(
                client.clone(),
                project_table_name,
            )),
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

fn extract_table_name<'a>(
    obj: &'a serde_json::Map<String, serde_json::Value>,
    key: &'a str,
) -> Result<&'a str, domain::Error> {
    let filed = obj.get(key).ok_or_else(|| {
        domain::Error::Configuration(format!(
            "Key '{}' was not found in the configuration file",
            key
        ))
    })?;

    let value = filed.as_str().ok_or_else(|| {
        domain::Error::Configuration(format!("Value for key '{}' is not a string", key))
    })?;

    Ok(value)
}
