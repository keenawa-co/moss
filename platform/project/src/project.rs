use anyhow::Result;
use app::{
    context::{AppContext, AsyncAppContext},
    event::{PlatformAction, PlatformEvent},
};
use async_utl::AsyncTryFrom;
use chrono::Utc;
use fs::FS;
use futures::stream::{select_all, Stream, StreamExt};
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
        ctx: &AsyncAppContext,
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
            worktree: Worktree::local(ctx, fs, &worktree_settings).await?,
            settings: Arc::new(initial_settings),
        })
    }

    pub async fn event_stream<'a>(
        self: &Arc<Self>,
    ) -> Pin<Box<dyn Send + Stream<Item = PlatformEvent<'a>> + 'a>> {
        let worktree_event_stream = self
            .worktree
            .event_stream()
            .await
            .map(|event| PlatformEvent {
                action: PlatformAction::new("project", "worktree", "fileCreated"),
                severity: app::event::Severity::Info,
                data: json!(event),
                timestamp: Utc::now().timestamp(),
            });

        Box::pin(select_all(vec![Box::pin(worktree_event_stream)]))
    }
}
