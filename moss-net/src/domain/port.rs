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
    ) -> Result<Vec<RecentProject>, super::Error>;
}

#[async_trait]
pub(crate) trait ProjectRepository: Debug + Send + Sync {
    async fn create_project(&self, input: NewProjectInput) -> Result<Vec<Project>, super::Error>;
    async fn delete_by_id(&self, id: String) -> Result<Option<Project>, super::Error>;
}
