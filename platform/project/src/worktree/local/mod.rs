mod scanner;
pub(crate) mod settings;

use anyhow::{Context, Result};
use fs::FS;
use smol::{channel::Receiver as SmolReceiver, channel::Sender as SmolSender, stream::StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task;

use self::scanner::LocalWorktreeScanner;
use self::settings::LocalWorktreeSettings;
use crate::model::event::{FileSystemEvent, ScannerEvent, WorktreeEvent};
use crate::model::filetree::{LocalFiletreeEntry, LocalFiletreeEntryKind};
use crate::model::snapshot::Snapshot;

#[derive(Debug, Clone)]
struct LocalWorktreeState {
    snapshot: Arc<RwLock<Snapshot>>,
}

impl LocalWorktreeState {
    fn new(snapshot: Snapshot) -> Arc<Self> {
        Arc::new(Self {
            snapshot: Arc::new(RwLock::new(snapshot)),
        })
    }
}

#[derive(Debug)]
pub struct LocalWorktree {
    fs: Arc<dyn FS>,
    state: Arc<LocalWorktreeState>,
}

impl LocalWorktree {
    pub async fn new(fs: Arc<dyn FS>, settings: &LocalWorktreeSettings) -> Result<Arc<Self>> {
        let root_name = settings
            .abs_path
            .file_name()
            .map_or_else(String::new, |f| f.to_string_lossy().to_string());

        let root_metadata = fs
            .metadata(&settings.abs_path)
            .await
            .context("could not open the root of the working directory")?;

        let mut initial_snapshot = Snapshot {
            root_name,
            abs_path: settings.abs_path.clone(),
            tree_by_path: Default::default(),
            file_scan_exclusions: settings.monitoring_exclude_list.clone(),
        };

        if let Some(metadata) = root_metadata {
            initial_snapshot.tree_by_path.insert(
                settings.abs_path.to_path_buf(),
                LocalFiletreeEntry {
                    kind: LocalFiletreeEntryKind::PendingDir,
                    path: settings.abs_path.clone(),
                    modified: metadata.modified,
                    is_symlink: metadata.is_symlink,
                },
            );
        } else {
            return Err(anyhow!(
                "could not get {} metadata",
                quote! {settings.abs_path.to_path_buf().to_string_lossy()}
            ));
        }

        Ok(Arc::new(Self {
            fs,
            state: LocalWorktreeState::new(initial_snapshot),
        }))
    }

    pub async fn run(self: &Arc<Self>, event_chan_tx: SmolSender<WorktreeEvent>) -> Result<()> {
        let worktree = self.clone();
        let (sync_state_tx, sync_state_rx) = smol::channel::unbounded::<WorktreeEvent>();

        tokio::try_join!(
            worktree.run_background_scanner(sync_state_tx),
            worktree.run_background_event_handler(sync_state_rx, event_chan_tx),
        )?;

        Ok(())
    }

    async fn run_background_scanner(
        self: &Arc<Self>,
        sync_state_tx: SmolSender<WorktreeEvent>,
    ) -> Result<()> {
        let snapshot = self.state.snapshot.read().await.clone();
        let fs_clone = self.fs.clone();
        let abs_path_clone = Arc::clone(&snapshot.abs_path);
        let fs_event_stream = fs_clone
            .watch(&abs_path_clone, Duration::from_secs(1))
            .await;

        task::spawn(async move {
            let scanner = LocalWorktreeScanner::new(fs_clone, sync_state_tx, snapshot.clone());

            // TODO: send error event to event_chan_tx in case of error
            if let Err(e) = scanner.run(abs_path_clone, fs_event_stream).await {
                error!("Error in worktree scanner: {e}");
            }
        });

        Ok(())
    }

    async fn run_background_event_handler(
        self: &Arc<Self>,
        sync_state_rx: SmolReceiver<WorktreeEvent>,
        event_chan_tx: SmolSender<WorktreeEvent>,
    ) -> Result<()> {
        let worktree = self.clone();

        task::spawn(async move {
            while let Ok(event) = sync_state_rx.recv().await {
                match &event {
                    WorktreeEvent::FileSystem(FileSystemEvent::Created(ref e))
                    | WorktreeEvent::FileSystem(FileSystemEvent::Deleted(ref e))
                    | WorktreeEvent::FileSystem(FileSystemEvent::Modified(ref e)) => {
                        let mut snapshot_lock = worktree.state.snapshot.write().await;

                        snapshot_lock
                            .tree_by_path
                            .insert(e.path.to_path_buf(), e.clone());
                    }

                    WorktreeEvent::Scanner(ScannerEvent::Discovered(ref entries)) => {
                        let mut snapshot_lock = worktree.state.snapshot.write().await;

                        for e in entries {
                            snapshot_lock
                                .tree_by_path
                                .insert(e.path.to_path_buf(), e.clone());
                        }
                    }
                }

                if let Err(e) = event_chan_tx.send(event).await {
                    error!("Failed to send worktree event: {e}");
                }
            }
        });

        Ok(())
    }
}
