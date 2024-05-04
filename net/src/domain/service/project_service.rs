use std::{path::PathBuf, sync::Arc};

use crate::domain::{model::result::Result, port::IgnoreListRepository};

pub struct ProjectService {
    ignore_repo: Arc<dyn IgnoreListRepository>,
}

impl ProjectService {
    pub fn new(ignore_repo: Arc<dyn IgnoreListRepository>) -> Self {
        Self {
            ignore_repo: ignore_repo,
        }
    }

    pub async fn create_ignore_list(&self, input_list: &Vec<PathBuf>) -> Result<()> {
        self.ignore_repo.create(input_list).await?;
        Ok(())
    }
}
