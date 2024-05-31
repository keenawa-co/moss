use anyhow::Result;
use async_utl::AsyncTryFrom;
use fs::FS;
use futures::stream::{select_all, Stream, StreamExt};
use platform_shared::model::event::AbstractEvent;
use serde_json::json;

use std::{path::Path, pin::Pin, sync::Arc};
use types::file::json_file::JsonFile;

use crate::{
    settings::Settings,
    worktree::{settings::LocalWorktreeSettings, Worktree},
};

#[derive(Debug)]
pub struct Project {
    pub worktree: Worktree,
    pub settings: Arc<Settings>,
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

        Ok(Self {
            worktree: Worktree::local(fs, &worktree_settings).await?,
            settings: Arc::new(initial_settings),
        })
    }

    pub async fn event_stream(&self) -> impl Stream<Item = AbstractEvent> {
        let worktree_event_stream = self
            .worktree
            .event_stream()
            .await
            .map(|event| AbstractEvent {
                route: "/workspace/project/worktree".to_string(),
                data: json!(event),
            });

        select_all(vec![Box::pin(worktree_event_stream)])
    }
}
