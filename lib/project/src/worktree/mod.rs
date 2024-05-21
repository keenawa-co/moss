pub mod local;
pub mod tree;

use fs::FS;
use std::sync::Arc;

use self::local::LocalWorktree;
use self::local::LocalWorktreeSettings;

#[derive(Debug)]
pub enum Worktree {
    Local(Arc<LocalWorktree>),
}

impl Worktree {
    pub async fn local(fs: Arc<dyn FS>, settings: &LocalWorktreeSettings) -> Self {
        Worktree::Local(LocalWorktree::new(fs, settings).await)
    }
}
