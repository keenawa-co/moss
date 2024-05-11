use common::id::NanoId;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::model::{
    error::Error, project::ProjectMeta, result::Result, session::SessionEntity, OptionExtension,
};
use crate::domain::port::rootdb::{ProjectMetaRepository, SessionRepository};
use crate::infra::adapter::sqlite::{CacheMigrator, CacheSQLiteAdapter};

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
        project_source: &PathBuf,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> Result<SessionEntity> {
        let project_meta = self
            .project_meta_repo
            .get_by_source(&project_source.canonicalize()?)
            .await?
            .ok_or_resource_not_found(
                &format!(
                    "project with source {} does not exist",
                    &project_source.to_string_lossy().to_string()
                ),
                None,
            )?;
        let session_info_entity = self.session_repo.create(&project_meta.id).await?;

        if !project_source.exists() {
            return Err(Error::resource_not_found(
                &format!(
                    "project {} is not found on your filesystem",
                    &project_source.to_string_lossy().to_string()
                ),
                None,
            ));
        }

        self.prepare_data(project_service, &project_meta).await?;

        Ok(SessionEntity {
            id: session_info_entity.id,
            project_meta: Some(project_meta),
            created_at: session_info_entity.created_at,
        })
    }

    pub async fn restore_session(
        &self,
        session_id: NanoId,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> Result<SessionEntity> {
        let session_entity = self
            .session_repo
            .get_by_id(&session_id)
            .await?
            .ok_or_resource_not_found(&format!("session {} does not exist", session_id), None)?;

        let project_meta = session_entity
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

        self.prepare_data(project_service, project_meta).await?;

        Ok(session_entity)
    }

    pub async fn get_recent_list(&self, start_time: i64, limit: u64) -> Result<Vec<SessionEntity>> {
        Ok(self
            .session_repo
            .fetch_list_by_start_time(start_time, limit)
            .await?)
    }
}

impl SessionService {
    async fn prepare_data(
        &self,
        project_service: &RwLock<Option<ProjectService>>,
        project_meta: &ProjectMeta,
    ) -> Result<()> {
        let project_db_client = {
            let project_path = PathBuf::from(&project_meta.source); // FIXME: avoid duplication (the same operation is performed in the parent function when checking the existence of a directory)
            let conn = dbutl::sqlite::conn::<CacheMigrator>(
                &project_path
                    .join(&self.conf.project_dir)
                    .join(&self.conf.project_db_file),
            )
            .await?;

            CacheSQLiteAdapter::new(Arc::new(conn))
        };

        let mut project_service_lock = project_service.write().await;
        *project_service_lock = Some(ProjectService::new(project_db_client.watch_list_repo()));

        Ok(())
    }
}
