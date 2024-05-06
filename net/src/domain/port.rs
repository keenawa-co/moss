use common::{id::NanoId, thing::Thing};
use std::{fmt::Debug, path::PathBuf};

use super::model::{
    project::{CreateProjectInput, IgnoredSource, ProjectMeta},
    result::Result,
    session::{Session, SessionInfo},
};

#[async_trait]
pub(crate) trait ProjectMetaRepository: Debug + Send + Sync {
    async fn create(&self, input: &CreateProjectInput) -> Result<ProjectMeta>;
    async fn get_by_id(&self, id: NanoId) -> Result<Option<ProjectMeta>>;
    async fn get_by_source(&self, source: PathBuf) -> Result<Option<ProjectMeta>>;
    async fn get_list_by_ids(&self, ids: &Vec<NanoId>) -> Result<Vec<ProjectMeta>>;
    async fn delete_by_id(&self, id: NanoId) -> Result<Option<Thing>>;
}

#[async_trait]
pub(crate) trait SessionRepository: Debug + Send + Sync {
    async fn create(&self, project_id: NanoId) -> Result<SessionInfo>;
    async fn get_by_id(&self, session_id: NanoId) -> Result<Option<Session>>;
    async fn get_recent_list(&self, start_time: i64, limit: u64) -> Result<Vec<Session>>;
}

#[async_trait]
pub(crate) trait IgnoreListRepository: Debug + Send + Sync {
    async fn create(&self, input_list: &Vec<PathBuf>) -> Result<Vec<IgnoredSource>>;
}
