use fs::real;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        self,
        model::session::{CreateSessionInput, Session, SessionInfo},
        port::{ProjectMetaRepository, SessionRepository},
    },
    infra::database::sqlite::{ProjectDatabaseClient, ProjectMigrator},
    not_found,
};

use super::ProjectService;

pub struct SessionService {
    realfs: Arc<real::FileSystem>,
    session_repo: Arc<dyn SessionRepository>,
    project_meta_repo: Arc<dyn ProjectMetaRepository>,
}

impl SessionService {
    pub fn new(
        realfs: Arc<real::FileSystem>,
        session_repo: Arc<dyn SessionRepository>,
        project_meta_repo: Arc<dyn ProjectMetaRepository>,
    ) -> Self {
        Self {
            realfs,
            project_meta_repo,
            session_repo,
        }
    }
}

impl SessionService {
    pub async fn create_session(
        &self,
        input: &CreateSessionInput,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> domain::Result<SessionInfo> {
        let project_meta_entity = self
            .project_meta_repo
            .get_by_source(input.project_source.canonicalize()?)
            .await?
            .ok_or_else(|| {
                not_found!(
                    "project with source {} does not exist",
                    input.project_source
                )
            })?;
        let session_entity = self.session_repo.create(project_meta_entity.id).await?;

        let project_path = PathBuf::from(&input.project_source);
        if !project_path.exists() {
            return Err(not_found!(
                "project {} is not found on your filesystem",
                input.project_source
            ));
        }

        {
            let project_db_client = {
                let project_path = pwd::init::create_from_scratch(
                    &PathBuf::from(&format!("{}/.moss", project_meta_entity.source)),
                    &self.realfs,
                )
                .await?;
                let conn = dbutl::sqlite::conn::<ProjectMigrator>(&project_path.join("project.db"))
                    .await?;

                ProjectDatabaseClient::new(Arc::new(conn))
            };

            let mut project_service_lock = project_service.write().await;
            *project_service_lock = Some(ProjectService::new(project_db_client.watch_list_repo()));
        }

        Ok(session_entity)
    }

    pub async fn get_recent_list(
        &self,
        start_time: i64,
        limit: u64,
    ) -> domain::Result<Vec<Session>> {
        Ok(self.session_repo.get_recent_list(start_time, limit).await?)
    }
}
