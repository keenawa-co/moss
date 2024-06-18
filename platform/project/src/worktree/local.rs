use anyhow::{Context as AnyhowContext, Result};
use app::context::{AppContext, AsyncAppContext};
use fs::FS;
use parking_lot::RwLock;
use smol::{channel::Receiver as SmolReceiver, channel::Sender as SmolSender};
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

use crate::model::event::{SharedWorktreeEntry, SharedWorktreeEvent};

use super::event::WorktreeEvent;
use super::filetree::{FileTreeEntryKind, FiletreeEntry};
use super::scanner::FileSystemScanService;
use super::settings::LocalWorktreeSettings;
use super::snapshot::Snapshot;

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
                FiletreeEntry {
                    kind: FileTreeEntryKind::Dir,
                    path: settings.abs_path.clone(),
                    modified: Some(metadata.modified),
                    is_symlink: Some(metadata.is_symlink),
                },
            );
        } else {
            return Err(anyhow!(
                "could not get {} metadata",
                quote!(settings.abs_path.to_path_buf().to_string_lossy())
            ));
        }

        Ok(Arc::new(Self {
            fs,
            state: LocalWorktreeState::new(initial_snapshot),
        }))
    }

    pub async fn run(
        self: &Arc<Self>,
        ctx: &AsyncAppContext,
        event_chan_tx: SmolSender<SharedWorktreeEvent>,
    ) -> Result<()> {
        let worktree = self.clone();

        let test_hook = |e: &mut WorktreeEvent| -> Result<()> {
            dbg!(1);
            let mut snapshot_lock = worktree.state.snapshot.write();

            match e {
                WorktreeEvent::Created(ref content) => {
                    let shared_event = SharedWorktreeEvent::Created(
                        content
                            .iter()
                            .map(|item| SharedWorktreeEntry {
                                path: item.path.clone(),
                                is_dir: item.is_dir(),
                            })
                            .collect::<Vec<SharedWorktreeEntry>>(),
                    );

                    for e in content {
                        snapshot_lock
                            .tree_by_path
                            .insert(e.path.to_path_buf(), e.clone());
                    }
                }
                WorktreeEvent::Deleted(ref e) => todo!(),
                WorktreeEvent::Modified(ref e) => {
                    todo!()
                }
            };

            Ok(())
        };

        // ctx.with_event_registry_mut(|registry| {
        //     registry.register_event::<WorktreeEvent>();
        //     registry.register_hook(test_hook);
        // });

        let worktree = self.clone();
        let (sync_state_tx, sync_state_rx) = smol::channel::unbounded::<WorktreeEvent>();

        // tokio::try_join!(
        //     worktree.run_background_scanner(sync_state_tx),
        //     worktree.run_background_event_handler(sync_state_rx, event_chan_tx),
        // )?;

        ctx.spawn(|ctx: &AsyncAppContext| {
            let worktree = self.clone();
            let ctx_clone = ctx.clone();

            async move {
                worktree
                    .run_background_scanner(ctx_clone, sync_state_tx)
                    .await
                    .unwrap();
            }
        })
        .detach();

        // ctx.spawn(|c| {
        //     let worktree = self.clone();
        //     async move {
        //         worktree
        //             .run_background_event_handler(sync_state_rx, event_chan_tx)
        //             .await
        //             .unwrap();
        //     }
        // })
        // .detach();

        Ok(())
    }

    async fn run_background_scanner<'a>(
        self: &Arc<Self>,
        ctx: AsyncAppContext,
        sync_state_tx: SmolSender<WorktreeEvent>,
    ) -> Result<()> {
        let snapshot = self.state.snapshot.read().clone();
        let fs_clone = self.fs.clone();
        let abs_path_clone = Arc::clone(&snapshot.abs_path);
        let fs_event_stream = fs_clone
            .watch(&abs_path_clone, Duration::from_secs(1))
            .await;

        ctx.spawn(|ctx| async move {
            let scanner = FileSystemScanService::new(fs_clone, sync_state_tx, snapshot.clone());
            // TODO: send error event to event_chan_tx in case of error
            if let Err(e) = scanner.run(abs_path_clone, fs_event_stream).await {
                error!("Error in worktree scanner: {e}");
            }
        })
        .detach();

        // task::spawn(async move {
        //     let scanner = FileSystemScanService::new(fs_clone, sync_state_tx, snapshot.clone());

        //     // TODO: send error event to event_chan_tx in case of error
        //     if let Err(e) = scanner.run(abs_path_clone, fs_event_stream).await {
        //         error!("Error in worktree scanner: {e}");
        //     }
        // });

        Ok(())
    }

    async fn run_background_event_handler(
        self: &Arc<Self>,
        sync_state_rx: SmolReceiver<WorktreeEvent>,
        event_chan_tx: SmolSender<SharedWorktreeEvent>,
    ) -> Result<()> {
        let worktree = self.clone();

        // task::spawn(async move {
        //     while let Ok(event) = sync_state_rx.recv().await {
        //         match &event {
        //             WorktreeEvent::Created(ref content) => {
        //                 let mut snapshot_lock = worktree.state.snapshot.write().await;

        //                 let shared_event = SharedWorktreeEvent::Created(
        //                     content
        //                         .iter()
        //                         .map(|item| SharedWorktreeEntry {
        //                             path: item.path.clone(),
        //                             is_dir: item.is_dir(),
        //                         })
        //                         .collect::<Vec<SharedWorktreeEntry>>(),
        //                 );

        //                 if let Err(e) = event_chan_tx.send(shared_event).await {
        //                     error!("Failed to send worktree event: {e}");
        //                 }

        //                 for e in content {
        //                     snapshot_lock
        //                         .tree_by_path
        //                         .insert(e.path.to_path_buf(), e.clone());
        //                 }
        //             }
        //             WorktreeEvent::Deleted(ref e) => todo!(),
        //             WorktreeEvent::Modified(ref e) => {
        //                 todo!()
        //             }
        //         }
        //     }
        // });

        Ok(())
    }

    // fn handle_event( self: &Arc<Self>, event: WorktreeEvent)
}
