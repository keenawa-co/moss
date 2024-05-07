use common::id::NanoId;
use hashbrown::HashMap;
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{
        model::{
            error::Error,
            project::ProjectMeta,
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
    context: HashMap<String, Value>,
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
            context: HashMap::new(),
        }
    }
}

impl SessionService {
    // pub fn session_id(&self) -> &Option<NanoId> {
    //     let r = self.context.get("project_id");
    // }

    // pub fn project_id(&self) -> &Option<NanoId> {
    //     &self.project_id
    // }

    pub fn get_from_context(&self, key: &str) -> Option<&Value> {
        self.context.get(key)
    }

    pub async fn create_session(
        &mut self,
        input: &CreateSessionInput,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> Result<SessionInfo> {
        let project_meta = self
            .project_meta_repo
            .get_by_source(&input.project_source.canonicalize()?)
            .await?
            .ok_or_resource_not_found(
                &format!(
                    "project with source {} does not exist",
                    input.project_source
                ),
                None,
            )?;
        let session_entity = self.session_repo.create(&project_meta.id).await?;

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

        self.prepare_data(project_service, &project_meta, session_entity.id.clone())
            .await?;

        Ok(session_entity)
    }

    pub async fn restore_session(
        &mut self,
        session_id: NanoId,
        project_service: &RwLock<Option<ProjectService>>,
    ) -> Result<Session> {
        let session = self
            .session_repo
            .get_by_id(&session_id)
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

        self.prepare_data(project_service, project_meta, session.id.clone())
            .await?;

        Ok(session)
    }

    pub async fn get_recent_list(&self, start_time: i64, limit: u64) -> Result<Vec<Session>> {
        Ok(self
            .session_repo
            .fetch_list_by_start_time(start_time, limit)
            .await?)
    }
}

impl SessionService {
    async fn prepare_data(
        &mut self,
        project_service: &RwLock<Option<ProjectService>>,
        project_meta: &ProjectMeta,
        session_id: NanoId,
    ) -> Result<()> {
        self.context.insert(
            String::from("session_id"),
            Value::String(session_id.to_string()),
        );
        self.context.insert(
            String::from("project_id"),
            Value::String(project_meta.id.clone().to_string()),
        );

        let project_db_client = {
            let project_path = PathBuf::from(&project_meta.source); // FIXME: avoid duplication (the same operation is performed in the parent function when checking the existence of a directory)
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
