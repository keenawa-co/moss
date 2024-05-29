pub mod local;

use anyhow::Result;
use fs::FS;
use smol::channel::Sender as SmolSender;
use std::sync::Arc;

use crate::model::event::WorktreeEvent;

use self::local::settings::LocalWorktreeSettings;
use self::local::LocalWorktree;

#[derive(Debug)]
pub(crate) enum Worktree {
    Local(Arc<LocalWorktree>),
    Remote,
}

pub(crate) struct WorktreeCreateInput {
    pub settings: LocalWorktreeSettings,
    pub event_chan_tx: SmolSender<WorktreeEvent>,
}

impl Worktree {
    pub async fn local(fs: Arc<dyn FS>, input: WorktreeCreateInput) -> Result<Self> {
        let worktree = LocalWorktree::new(fs, &input.settings).await?;
        worktree.run(input.event_chan_tx).await?;

        Ok(Worktree::Local(worktree))
    }
}
