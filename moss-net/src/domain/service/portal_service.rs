use std::sync::Arc;

use crate::domain::{self, model::portal::RecentProject, port::PortalRepository};

#[derive(Debug, Clone)]
pub struct PortalService {
    pub repo: Arc<dyn PortalRepository>,
}

impl PortalService {
    pub fn new(repo: Arc<dyn PortalRepository>) -> Self {
        Self { repo }
    }

    pub async fn select_resent_list(
        &self,
        start_time: i64,
        limit: u8,
    ) -> domain::Result<Vec<RecentProject>> {
        let result = self.repo.select_resent_list(start_time, limit).await?;

        Ok(result)
    }
}
