use std::path::PathBuf;

use crate::settings::Settings;

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub settings: Settings,
}

impl Project {
    pub fn new(root_path: &PathBuf, settings: Settings) -> Self {
        Self {
            root: root_path.clone(),
            settings,
        }
    }
}
