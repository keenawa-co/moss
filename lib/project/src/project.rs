use anyhow::Result;
use async_utl::AsyncTryFrom;
use fs::FS;
use futures::Stream;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use types::file::json_file::JsonFile;

use crate::{settings::Settings, worktree::local::WorkTreeEvent, worktree::Worktree};

#[derive(Debug)]
pub struct Project {
    pub worktree: Worktree,
    pub settings: Settings,
}

impl Project {
    pub async fn new(
        fs: Arc<dyn FS>,
        dir: Arc<Path>,
        settings_file: Arc<JsonFile>,
    ) -> Result<Self> {
        Ok(Self {
            worktree: Worktree::local(fs, dir).await,
            settings: Settings::try_from_async(settings_file).await?,
        })
    }

    pub async fn worktree_event_stream(&self) -> impl Stream<Item = WorkTreeEvent> {
        match &self.worktree {
            Worktree::Local(local) => local.event_stream().await,
        }
    }
}
