use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use types::file::json_file::JsonFile;

use crate::domain::model::result::Result;

pub struct WorkspaceService {
    settings_file: Option<Arc<JsonFile>>,
}

pub struct CreateConfig<'a> {
    pub project_path: &'a PathBuf,
}

impl WorkspaceService {
    pub fn init() -> Self {
        Self {
            settings_file: None,
        }
    }

    pub async fn create<'a, 'b>(&'a mut self, conf: &'b CreateConfig<'b>) -> Result<()> {
        let settings_file = JsonFile::new(&conf.project_path.join(".moss/settings.json")).await?; // TODO: use WorkspaceConfig

        self.settings_file = Some(Arc::new(settings_file));

        Ok(())
    }
}

impl WorkspaceService {
    pub fn get_settings(&self) -> Option<Arc<JsonFile>> {
        self.settings_file.clone()
    }
}
