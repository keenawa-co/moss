use fs::real;
use std::sync::Arc;
use types::{id::NanoId, thing::Thing};

use crate::domain::{
    model::{
        project::{CreateProjectInput, ProjectMeta},
        result::Result,
        OptionExtension,
    },
    port::rootdb::ProjectMetaRepository,
};

#[derive(Debug, Clone)]
pub struct ProjectMetaService {
    realfs: Arc<real::FileSystem>,
    project_repo: Arc<dyn ProjectMetaRepository>,
}

impl ProjectMetaService {
    pub fn new(
        realfs: Arc<real::FileSystem>,
        project_repo: Arc<dyn ProjectMetaRepository>,
    ) -> Arc<Self> {
        Arc::new(Self {
            realfs,
            project_repo,
        })
    }

    pub async fn create_project(
        self: &Arc<Self>,
        input: &CreateProjectInput,
    ) -> Result<ProjectMeta> {
        let project_entity = self.project_repo.create(&input).await?;

        pwd::init::create_from_scratch(input.path.as_path_buf(), &self.realfs).await?;

        Ok(project_entity)
    }

    pub async fn delete_project_by_id(self: &Arc<Self>, id: &NanoId) -> Result<Thing<NanoId>> {
        let result = self
            .project_repo
            .delete_by_id(id)
            .await?
            .ok_or_resource_not_found(&format!("project with id {} does not exist", id), None)?;
        // code = ErrorCode::EXPECTED_BUT_NOT_FOUND

        Ok(result)
    }
}
