pub(crate) mod rootdb {
    use std::{fmt::Debug, path::PathBuf};
    use types::{id::NanoId, thing::Thing};

    use crate::domain::model::{
        project::{CreateProjectInput, ProjectMeta},
        result::Result,
        session::{SessionEntity, SessionInfoEntity},
    };

    #[async_trait]
    pub trait ProjectMetaRepository: Debug + Send + Sync {
        async fn create(&self, input: &CreateProjectInput) -> Result<ProjectMeta>;
        async fn get_by_id(&self, id: &NanoId) -> Result<Option<ProjectMeta>>;
        async fn get_by_source(&self, source: &PathBuf) -> Result<Option<ProjectMeta>>;
        async fn get_list_by_ids(&self, ids: &Vec<NanoId>) -> Result<Vec<ProjectMeta>>;
        async fn delete_by_id(&self, id: &NanoId) -> Result<Option<Thing<NanoId>>>;
    }

    #[async_trait]
    pub trait SessionRepository: Debug + Send + Sync {
        async fn create(&self, project_id: &NanoId) -> Result<SessionInfoEntity>;
        async fn get_by_id(&self, session_id: &NanoId) -> Result<Option<SessionEntity>>;
        async fn fetch_list_by_start_time(
            &self,
            start_time: i64,
            limit: u64,
        ) -> Result<Vec<SessionEntity>>;
    }
}
