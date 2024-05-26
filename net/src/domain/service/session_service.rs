use std::{path::PathBuf, sync::Arc};
use types::id::NanoId;

use crate::domain::model::{error::Error, result::Result, session::SessionEntity, OptionExtension};
use crate::domain::port::rootdb::{ProjectMetaRepository, SessionRepository};

pub struct SessionService {
    session_repo: Arc<dyn SessionRepository>,
    project_meta_repo: Arc<dyn ProjectMetaRepository>,
}

impl SessionService {
    pub fn new(
        session_repo: Arc<dyn SessionRepository>,
        project_meta_repo: Arc<dyn ProjectMetaRepository>,
    ) -> Arc<Self> {
        Arc::new(Self {
            project_meta_repo,
            session_repo,
        })
    }
}

impl SessionService {
    pub async fn create_session(self: &Arc<Self>, project_path: &PathBuf) -> Result<SessionEntity> {
        let project_meta = self
            .project_meta_repo
            .get_by_source(&project_path.canonicalize()?)
            .await?
            .ok_or_resource_not_found(
                &format!(
                    "project with source {} does not exist", // TODO: use quote! for {} value
                    &project_path.to_string_lossy().to_string()
                ),
                None,
            )?;
        let session_info_entity = self.session_repo.create(&project_meta.id).await?;

        if !project_path.exists() {
            return Err(Error::resource_not_found(
                &format!(
                    "project {} is not found on your filesystem", // TODO: use quote! for {} value
                    &project_path.to_string_lossy().to_string()
                ),
                None,
            ));
        }

        Ok(SessionEntity {
            id: session_info_entity.id,
            project_meta: Some(project_meta),
            created_at: session_info_entity.created_at,
        })
    }

    pub async fn restore_session(self: &Arc<Self>, session_id: NanoId) -> Result<SessionEntity> {
        let session_entity = self
            .session_repo
            .get_by_id(&session_id)
            .await?
            .ok_or_resource_not_found(&format!("session {} does not exist", session_id), None)?; // TODO: use quote! for {} value
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

        Ok(session_entity)
    }

    pub async fn get_recent_list(
        self: &Arc<Self>,
        start_time: i64,
        limit: u64,
    ) -> Result<Vec<SessionEntity>> {
        Ok(self
            .session_repo
            .fetch_list_by_start_time(start_time, limit)
            .await?)
    }
}
