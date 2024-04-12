use std::fmt::Debug;

use super::model::project::{NewProjectInput, Project, RecentProject};

#[async_trait]
pub(crate) trait ProjectRepository: Debug + Send + Sync {
    async fn create_project(&self, input: NewProjectInput) -> super::Result<Project>;
    async fn delete_by_id(&self, id: i32) -> super::Result<Option<Project>>;
    async fn select_resent_list(
        &self,
        start_time: i64,
        limit: u64,
    ) -> super::Result<Vec<RecentProject>>;
}
