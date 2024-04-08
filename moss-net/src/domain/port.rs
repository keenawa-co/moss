use async_trait::async_trait;
use std::fmt::Debug;

use super::model::{
    portal::RecentProject,
    project::{NewProjectInput, Project},
};

#[async_trait]
pub(crate) trait PortalRepository: Debug + Send + Sync {
    async fn select_resent_list(
        &self,
        start_time: i64,
        limit: u8,
    ) -> super::Result<Vec<RecentProject>>;
}

#[async_trait]
pub(crate) trait ProjectRepository: Debug + Send + Sync {
    async fn create_project(&self, input: NewProjectInput) -> super::Result<Vec<Project>>;
    async fn delete_by_id(&self, id: String) -> super::Result<Option<Project>>;
}
