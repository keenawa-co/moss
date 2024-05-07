use std::{path::PathBuf, sync::Arc};

use common::{id::NanoId, thing::Thing};

use crate::domain::{
    model::{project::IgnoredSource, result::Result, OptionExtension},
    port::IgnoreListRepository,
};

pub struct ProjectService {
    ignore_repo: Arc<dyn IgnoreListRepository>,
}

impl ProjectService {
    pub fn new(ignore_repo: Arc<dyn IgnoreListRepository>) -> Self {
        Self {
            ignore_repo: ignore_repo,
        }
    }

    pub async fn append_to_ignore_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> Result<Vec<IgnoredSource>> {
        Ok(self.ignore_repo.create_from_list(input_list).await?)
    }

    pub async fn remove_from_ignore_list(&self, id: &NanoId) -> Result<Thing> {
        let result = self
            .ignore_repo
            .delete_by_id(id)
            .await?
            .ok_or_resource_not_found(&format!("project with id {} does not exist", id), None)?;

        Ok(result)
    }
}
