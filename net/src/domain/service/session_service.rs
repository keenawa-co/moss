use common::id::MNID;
use fs::real;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        self,
        model::session::{self, CreateSessionInput, Session},
        port::{ProjectRepository, SessionRepository},
    },
    infra::database::sqlite::{ProjectDatabaseClient, ProjectMigrator},
    internal,
};

use super::SessionProjectService;

pub struct SessionService {
    realfs: Arc<real::FileSystem>,
    project_repo: Arc<dyn ProjectRepository>,
    session_repo: Arc<dyn SessionRepository>,
}

impl SessionService {
    pub fn new(
        realfs: Arc<real::FileSystem>,
        project_repo: Arc<dyn ProjectRepository>,
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
        session_project_service: &RwLock<Option<SessionProjectService>>,
    ) -> domain::Result<Session> {
        let project_entity = self
            .project_repo
            .get_project_by_id(input.project_id.to_string())
            .await?
            .ok_or_else(|| internal!("project with id {} not found", input.project_id))?;
        let session_entity = self.session_repo.create_session(input).await?;

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
            *session_project_service_lock = Some(SessionProjectService::new(
                project_db_client.watch_list_repo(),
            ));
        }

        Ok(session_entity)
    }
}
