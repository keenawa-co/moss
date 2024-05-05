use common::id::NanoId;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        model::{
            error::Error,
            result::Result,
            session::{CreateSessionInput, Session, SessionInfo},
            OptionExtension,
        },
        port::{ProjectMetaRepository, SessionRepository},
    },
    infra::database::sqlite::{ProjectDatabaseClient, ProjectMigrator},
};

use super::ProjectService;

pub struct SessionServiceConfig {
    // Path to the directory inside the project selected by the user for work.
    // It is assumed that a project creation operation has already been
    // performed for this project.
    pub project_dir: PathBuf,
    // Name of the database file created for each project in the local folder.
    pub project_db_file: PathBuf,
}

pub struct SessionService {
    session_repo: Arc<dyn SessionRepository>,
    project_meta_repo: Arc<dyn ProjectMetaRepository>,
    conf: SessionServiceConfig,
}

impl SessionService {
    pub fn new(
        session_repo: Arc<dyn SessionRepository>,
        project_meta_repo: Arc<dyn ProjectMetaRepository>,
        conf: SessionServiceConfig,
    ) -> Self {
        Self {
            project_meta_repo,
            session_repo,
            conf,
        }
    }
}

impl SessionService {
    pub async fn create_session(
        &self,
        input: &CreateSessionInput,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> Result<SessionInfo> {
        let project_meta_entity = self
            .project_meta_repo
            .get_by_source(input.project_source.canonicalize()?)
            .await?
            .ok_or_resource_not_found(
                &format!(
                    "project with source {} does not exist",
                    input.project_source
                ),
                None,
            )?;
        let session_entity = self.session_repo.create(project_meta_entity.id).await?;

        let project_path = PathBuf::from(&input.project_source);
        if !project_path.exists() {
            return Err(Error::resource_not_found(
                &format!(
                    "project {} is not found on your filesystem",
                    input.project_source
                ),
                None,
            ));
        }

        self.prepare_data(project_service, &project_path).await?;

        Ok(session_entity)
    }

    pub async fn restore_session(
        &self,
        session_id: NanoId,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> Result<Session> {
        let session = self
            .session_repo
            .get_by_id(session_id.clone())
            .await?
            .ok_or_resource_not_found(&format!("session {} does not exist", session_id), None)?;

        let project_meta = session
            .project_meta
            .as_ref()
            .ok_or_resource_invalid("session project does not exist", None)?;
        let project_path = PathBuf::from(&project_meta.source);

        if !project_path.exists() {
            return Err(Error::resource_invalid(
                &format!(
                    "project {} is not found on your filesystem",
                    project_meta.source
                ),
                None,
            ));
        }

        self.prepare_data(project_service, &project_path).await?;

        Ok(session)
    }

    pub async fn get_recent_list(&self, start_time: i64, limit: u64) -> Result<Vec<Session>> {
        Ok(self.session_repo.get_recent_list(start_time, limit).await?)
    }
}

impl SessionService {
    async fn prepare_data(
        &self,
        project_service: &RwLock<Option<ProjectService>>,
        project_path: &PathBuf,
    ) -> Result<()> {
        let project_db_client = {
            let conn = dbutl::sqlite::conn::<ProjectMigrator>(
                &project_path
                    .join(&self.conf.project_dir)
                    .join(&self.conf.project_db_file),
            )
            .await?;

            ProjectDatabaseClient::new(Arc::new(conn))
        };

        let mut project_service_lock = project_service.write().await;
        *project_service_lock = Some(ProjectService::new(project_db_client.watch_list_repo()));

        Ok(())
    }
}
