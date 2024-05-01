use common::{id::NanoId, thing::Thing};
use std::fmt::Debug;

use super::model::{
    project::{CreateProjectInput, ProjectMeta},
    session::{Session, SessionInfo},
};

#[async_trait]
pub(crate) trait ProjectMetaRepository: Debug + Send + Sync {
    async fn create(&self, input: &CreateProjectInput) -> super::Result<ProjectMeta>;
    async fn get_by_id(&self, id: NanoId) -> super::Result<Option<ProjectMeta>>;
    async fn get_by_source(&self, source: String) -> super::Result<Option<ProjectMeta>>;
    async fn get_list_by_ids(&self, ids: &Vec<NanoId>) -> super::Result<Vec<ProjectMeta>>;
    async fn delete_by_id(&self, id: NanoId) -> super::Result<Thing>;
}

#[async_trait]
pub(crate) trait SessionRepository: Debug + Send + Sync {
    async fn create(&self, project_id: NanoId) -> super::Result<SessionInfo>;
    async fn get_recent_list(&self, start_time: i64, limit: u64) -> super::Result<Vec<Session>>;
}

#[async_trait]
pub(crate) trait IgnoreRepository: Debug + Send + Sync {}
