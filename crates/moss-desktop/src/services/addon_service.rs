use anyhow::Result;
use std::path::PathBuf;

use crate::{addon_registry::AddonRegistry, state::AppState};

pub struct AddonService {
    addons: AddonRegistry,
    builtin_addons_location: PathBuf,
    installed_addons_location: PathBuf,
}

impl AddonService {
    pub fn new(builtin_addons_location: PathBuf, installed_addons_location: PathBuf) -> Self {
        Self {
            addons: AddonRegistry::new(),
            builtin_addons_location,
            installed_addons_location,
        }
    }

    // pub async fn initialize(&self, app_state: &AppState) -> Result<()> {
    //     // let read_dir = tokio::fs::read_dir(self.builtin_addons_location).await?;

    //     while let Some(entry) = read_dir.next_entry().await? {}
    // }
}
