use common::{id::NanoId, thing::Thing};
use fs::real;
use std::sync::Arc;

use crate::domain::{
    model::{
        project::{CreateProjectInput, ProjectMeta},
        result::Result,
        OptionExtension,
    },
    port::ProjectMetaRepository,
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
    ) -> Self {
        Self {
            realfs,
            project_repo,
        }
    }

    pub async fn create_project(&self, input: &CreateProjectInput) -> Result<ProjectMeta> {
        let project_entity = self.project_repo.create(&input).await?;

        pwd::init::create_from_scratch(input.path.as_path_buf(), &self.realfs).await?;

        Ok(project_entity)
    }

    pub async fn delete_project_by_id(&self, id: &NanoId) -> Result<Thing> {
        let result = self
            .project_repo
            .delete_by_id(id)
            .await?
            .ok_or_resource_not_found(&format!("project with id {} does not exist", id), None)?;
        // code = ErrorCode::EXPECTED_BUT_NOT_FOUND

        Ok(result)
    }
}
