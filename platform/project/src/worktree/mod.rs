pub mod settings;

mod event;
mod filetree;
mod local;
mod scanner;
mod snapshot;

use anyhow::Result;
use fs::FS;
use smol::channel::Sender as SmolSender;
use std::sync::Arc;

use crate::model::event::SharedEvent;

use self::{local::LocalWorktree, settings::LocalWorktreeSettings};

#[derive(Debug)]
pub(crate) enum Worktree {
    Local(Arc<LocalWorktree>),
    Remote,
}

pub(crate) struct WorktreeCreateInput {
    pub settings: LocalWorktreeSettings,
    // pub event_chan_tx: SmolSender<WorktreeEvent>,
}

impl Worktree {
    pub async fn local(
        fs: Arc<dyn FS>,
        settings: &LocalWorktreeSettings,
        event_chan_tx: SmolSender<SharedEvent>,
    ) -> Result<Self> {
        let worktree = LocalWorktree::new(fs, settings).await?;
        worktree.run(event_chan_tx).await?;

        Ok(Worktree::Local(worktree))
    }
}
