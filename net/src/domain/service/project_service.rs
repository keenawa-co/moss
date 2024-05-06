use std::{path::PathBuf, sync::Arc};

use crate::domain::{
    model::{project::IgnoredSource, result::Result},
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
        Ok(self.ignore_repo.create(input_list).await?)
    }
}
