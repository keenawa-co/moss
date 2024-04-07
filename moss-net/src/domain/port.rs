use async_trait::async_trait;
use std::fmt::Debug;

use super::model::portal::{RecentItem, RecentItemInput};

#[async_trait]
pub trait PortalRepository: Debug + Send + Sync {
    async fn select_resent_list(&self) -> Result<Vec<RecentItem>, super::Error>;
    async fn create_resent(&self, item: RecentItemInput) -> Result<Vec<RecentItem>, super::Error>;
    async fn delete_recent_by_id(&self, path: String) -> Result<Option<RecentItem>, super::Error>;
}
