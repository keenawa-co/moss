use std::{path::PathBuf, sync::Arc};

use crate::domain::{self, port::IgnoreRepository};

pub struct ProjectService {
    ignore_repo: Arc<dyn IgnoreRepository>,
}

impl ProjectService {
    pub fn new(ignore_repo: Arc<dyn IgnoreRepository>) -> Self {
        Self {
            ignore_repo: ignore_repo,
        }
    }

    pub async fn create_ignore_list(&self, input_list: &Vec<PathBuf>) -> domain::Result<()> {
        self.ignore_repo.create(input_list).await?;
        Ok(())
    }
}
