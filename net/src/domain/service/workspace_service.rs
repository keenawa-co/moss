use arc_swap::ArcSwapOption;
use std::{path::PathBuf, sync::Arc};
use types::file::json_file::JsonFile;

use crate::domain::model::result::Result;

pub struct WorkspaceService {
    settings_file: ArcSwapOption<JsonFile>,
}

pub struct CreateConfig<'a> {
    pub project_path: &'a PathBuf,
}

impl WorkspaceService {
    pub fn init() -> Arc<Self> {
        Arc::new(Self {
            settings_file: ArcSwapOption::from(None),
        })
    }

    pub async fn create<'a>(self: &Arc<Self>, conf: &CreateConfig<'a>) -> Result<()> {
        let settings_file = JsonFile::new(&conf.project_path.join(".moss/settings.json")).await?; // TODO: use WorkspaceConfig
        self.settings_file.store(Some(Arc::new(settings_file)));

        Ok(())
    }
}

impl WorkspaceService {
    pub fn get_settings(&self) -> Option<Arc<JsonFile>> {
        self.settings_file.load_full()
    }
}
