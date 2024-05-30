use anyhow::Result;
use async_utl::AsyncTryFrom;
use fs::FS;
use futures::Stream;
use platform_shared::model::event::AbstractEvent;
use serde_json::json;
use smol::channel::Receiver as SmolReceiver;
use std::{path::Path, sync::Arc};
use types::file::json_file::JsonFile;

use crate::{
    model::event::SharedEvent,
    settings::Settings,
    worktree::{settings::LocalWorktreeSettings, Worktree, WorktreeCreateInput},
};

#[derive(Debug)]
pub struct Project {
    pub worktree: Worktree,
    pub settings: Arc<Settings>,

    event_chan_rx: SmolReceiver<SharedEvent>,
}

impl Project {
    pub async fn new(
        fs: Arc<dyn FS>,
        dir_abs_path: Arc<Path>,
        settings_file: Arc<JsonFile>,
    ) -> Result<Self> {
        let initial_settings = Settings::try_from_async(settings_file).await?;
        let worktree_settings = LocalWorktreeSettings {
            abs_path: dir_abs_path.clone(),
            monitoring_exclude_list: Arc::new(initial_settings.fetch_exclude_list()),
            watch_gitignore_entries: initial_settings.watch_gitignore_entries,
            auto_watch_new_entries: initial_settings.auto_watch_new_entries,
        };

        let (event_chan_tx, event_chan_rx) = smol::channel::unbounded();

        Ok(Self {
            worktree: Worktree::local(fs, &worktree_settings, event_chan_tx).await?,
            settings: Arc::new(initial_settings),
            event_chan_rx,
        })
    }

    pub async fn event_stream(&self) -> impl Stream<Item = AbstractEvent> {
        let event_chan_rx = self.event_chan_rx.clone();

        futures::stream::unfold(event_chan_rx, |receiver| async {
            match receiver.recv().await {
                Ok(event) => Some((
                    AbstractEvent {
                        route: "test".to_string(),
                        data: json!(event),
                    },
                    receiver,
                )),
                Err(_) => None,
            }
        })
    }
}
