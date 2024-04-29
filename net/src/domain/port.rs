use common::{id::MNID, thing::Thing};
use std::fmt::Debug;

use super::model::{
    project::{CreateProjectInput, Project},
    session::{CreateSessionInput, Session},
};

#[async_trait]
pub(crate) trait ProjectRepository: Debug + Send + Sync {
    async fn create(&self, input: &CreateProjectInput) -> super::Result<Project>;
    async fn get_by_id(&self, id: MNID) -> super::Result<Option<Project>>;
    async fn get_list_by_ids(&self, ids: &Vec<MNID>) -> super::Result<Vec<Project>>;
    async fn delete_by_id(&self, id: MNID) -> super::Result<Thing>;
}

#[async_trait]
pub(crate) trait SessionRepository: Debug + Send + Sync {
    async fn create(&self, input: &CreateSessionInput) -> super::Result<Session>;
    async fn get_recent_list(&self, start_time: i64, limit: u64) -> super::Result<Vec<Session>>;
}

#[async_trait]
pub(crate) trait WatchListRepository: Debug + Send + Sync {}
