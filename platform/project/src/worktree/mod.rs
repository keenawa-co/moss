pub mod local;

use anyhow::Result;
use async_graphql::Object;
use fs::FS;
use std::sync::Arc;

use self::local::event::{FileSystemEvent, ScannerEvent};
use self::local::LocalWorktree;
use self::local::LocalWorktreeSettings;

#[derive(Debug)]
pub enum Worktree {
    Local(LocalWorktree),
    Remote,
}

impl Worktree {
    pub async fn local(fs: Arc<dyn FS>, settings: &LocalWorktreeSettings) -> Result<Self> {
        let worktree = LocalWorktree::new(fs, settings).await?;
        worktree.run().await?;

        Ok(Worktree::Local(worktree))
    }
}
