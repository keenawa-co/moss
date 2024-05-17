use anyhow::Result;
use async_utl::AsyncTryFrom;
use std::{path::PathBuf, sync::Arc};
use types::file::json_file::JsonFile;

use crate::settings::Settings;

#[derive(Debug)]
pub struct Project {
    pub dir: PathBuf,
    pub settings: Settings,
}

impl Project {
    pub async fn new(dir: &PathBuf, settings_file: Arc<JsonFile>) -> Result<Self> {
        let settings = Settings::try_from_async(settings_file).await?;
        Ok(Self {
            dir: dir.to_owned(),
            settings,
        })
    }
}
