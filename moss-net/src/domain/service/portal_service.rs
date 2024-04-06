use std::sync::Arc;

use crate::domain::{
    self,
    model::portal::{RecentItem, RecentItemInput},
    port::PortalRepository,
};

#[derive(Debug, Clone)]
pub struct PortalService {
    pub cache: Arc<dyn PortalRepository>,
}

impl PortalService {
    pub fn new(cache: Arc<dyn PortalRepository>) -> Self {
        Self { cache }
    }

    pub async fn select_resent_list(&self) -> Result<Vec<RecentItem>, domain::Error> {
        let result = self.cache.select_resent_list().await?;

        Ok(result)
    }

    pub async fn crate_recent(
        &self,
        item: RecentItemInput,
    ) -> Result<Vec<RecentItem>, domain::Error> {
        let result = self.cache.create_resent(item).await?;

        Ok(result) // FIXME: return one item, not a vector
    }

    pub async fn delete_by_id(&self, path: String) -> Result<RecentItem, domain::Error> {
        let result = self.cache.delete_by_id(path).await?;

        Ok(result.unwrap())
    }
}
