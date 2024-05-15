use std::path::PathBuf;

use crate::settings::ProjectSettings;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub settings: ProjectSettings,
}

impl Project {
    pub fn new(root_path: &PathBuf, settings: ProjectSettings) -> Self {
        Self {
            root: root_path.clone(),
            settings,
        }
    }
}
