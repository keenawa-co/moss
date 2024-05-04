use common::{id::NanoId, thing::Thing};
use fs::real;
use std::{path::PathBuf, sync::Arc};

use crate::{
    domain::{
        self,
        model::{
            project::{CreateProjectInput, ProjectMeta},
            result::ErrorCode,
        },
        port::ProjectMetaRepository,
    },
    internal, not_found,
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

    pub async fn create_project(&self, input: &CreateProjectInput) -> domain::Result<ProjectMeta> {
        let project_entity = self.project_repo.create(&input).await?;

        pwd::init::create_from_scratch(input.path.as_path_buf(), &self.realfs).await?;

        Ok(project_entity)
    }

    pub async fn delete_project_by_id(&self, id: NanoId) -> domain::Result<Thing> {
        let result = self
            .project_repo
            .delete_by_id(id.clone())
            .await?
            .ok_or_else(|| not_found!("project with id {} does not exist", id))?;
        // code = ErrorCode::EXPECTED_BUT_NOT_FOUND

        Ok(result)
    }

    pub async fn get_project_list_by_ids(
        &self,
        ids: &Vec<NanoId>,
    ) -> domain::Result<Vec<ProjectMeta>> {
        Ok(self.project_repo.get_list_by_ids(ids).await?)
    }
}
