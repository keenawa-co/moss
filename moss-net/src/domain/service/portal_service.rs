use std::sync::Arc;

use crate::domain::{
    self,
    model::portal::{RecentItem, RecentItemInput},
    port::PortalRepository,
};

#[derive(Debug, Clone)]
pub struct PortalService {
    pub repo: Arc<dyn PortalRepository>,
}

impl PortalService {
    pub fn new(repo: Arc<dyn PortalRepository>) -> Self {
        Self { repo }
    }

    pub async fn select_resent_list(&self) -> Result<Vec<RecentItem>, domain::Error> {
        let result = self.repo.select_resent_list().await?;

        Ok(result)
    }

    pub async fn crate_recent(
        &self,
        item: RecentItemInput,
    ) -> Result<Vec<RecentItem>, domain::Error> {
        let result = self.repo.create_resent(item).await?;

        Ok(result)
    }

    pub async fn delete_recent_by_id(&self, path: String) -> Result<RecentItem, domain::Error> {
        let result = self.repo.delete_recent_by_id(path).await?;

        Ok(result.unwrap())
    }
}
