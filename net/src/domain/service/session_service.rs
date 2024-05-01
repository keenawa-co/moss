use fs::real;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        self,
        model::session::{CreateSessionInput, Session},
        port::{ProjectMetaRepository, SessionRepository},
    },
    infra::database::sqlite::{ProjectDatabaseClient, ProjectMigrator},
    internal,
};

use super::ProjectService;

pub struct SessionService {
    realfs: Arc<real::FileSystem>,
    project_repo: Arc<dyn ProjectMetaRepository>,
    session_repo: Arc<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(
        realfs: Arc<real::FileSystem>,
        project_repo: Arc<dyn ProjectMetaRepository>,
        session_repo: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            realfs,
            project_repo,
            session_repo,
        }
    }
}

impl SessionService {
    pub async fn create_session(
        &self,
        input: &CreateSessionInput,
        session_project_service: &RwLock<Option<ProjectService>>,
    ) -> domain::Result<Session> {
        let project_entity = self
            .project_repo
            .get_by_source(input.project_source.clone())
            .await?
            .ok_or_else(|| internal!("project with source {} not found", input.project_source))?;
        let session_entity = self.session_repo.create(project_entity.id).await?;

        let project_path = PathBuf::from(&input.project_source);
        if !project_path.exists() {
            return Err(internal!(
                "project with source {} is exists in the database but not found on your filesystem",
                input.project_source
            ));
        }

        {
            let project_db_client = {
                let project_path = pwd::init::create_from_scratch(
                    &PathBuf::from(&format!("{}/.moss", project_entity.source)),
                    &self.realfs,
                )
                .await?;
                let conn = dbutl::sqlite::conn::<ProjectMigrator>(&project_path.join("project.db"))
                    .await?;

                ProjectDatabaseClient::new(Arc::new(conn))
            };

            let mut session_project_service_lock = session_project_service.write().await;
            *session_project_service_lock =
                Some(ProjectService::new(project_db_client.watch_list_repo()));
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
