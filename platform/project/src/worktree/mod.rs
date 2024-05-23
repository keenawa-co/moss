pub mod event;
pub mod local;
pub mod tree;

use anyhow::Result;
use fs::FS;
use std::sync::Arc;

use self::local::LocalWorktree;
use self::local::LocalWorktreeSettings;

#[derive(Debug)]
pub enum Worktree {
    Local(LocalWorktree),
    Remote,
}

impl Worktree {
    pub async fn local(fs: Arc<dyn FS>, settings: &LocalWorktreeSettings) -> Result<Self> {
        Ok(Worktree::Local(LocalWorktree::new(fs, settings).await?))
    }
}
