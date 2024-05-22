use anyhow::{Context, Result};
use fs::FS;
use futures::task::Poll;
use futures::{select_biased, FutureExt, Stream};
use hashbrown::HashSet;
use smol::stream::StreamExt;
use std::time::{Duration, SystemTime};
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::RwLock;
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

use super::tree::{FileTree, Snapshot, WorktreeEntry};
use crate::worktree::tree::WorktreeEntryKind;

pub struct LocalWorktreeSettings {
    pub abs_path: Arc<Path>,
    pub monitoring_exclude_list: Arc<HashSet<PathBuf>>,
    pub watch_gitignore_entries: bool,
    pub auto_watch_new_entries: bool,
}

#[derive(Debug)]
pub struct WorktreeScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<WorktreeScanJob>,
}

#[derive(Debug, Serialize, Clone)]
pub enum WorkTreeEventKind {
    Create,
    Delete,
    Modify,
    Discovery,
}

#[derive(Debug, Clone)]
pub struct WorktreeEvent {
    pub kind: WorkTreeEventKind,
    pub entry: WorktreeEntry,
}

#[derive(Debug, Clone)]
pub struct LocalWorktreeState {
    prev_snapshot: Option<Snapshot>,
    last_snapshot: Snapshot,
}

#[derive(Debug)]
pub struct LocalWorktree {
    state: Arc<RwLock<LocalWorktreeState>>,
    share_rx: Arc<Mutex<mpsc::UnboundedReceiver<WorktreeEvent>>>,
}

impl LocalWorktree {
    pub async fn new(fs: Arc<dyn FS>, settings: &LocalWorktreeSettings) -> Result<Self> {
        let root_name = settings
            .abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());

        let root_metadata = fs
            .metadata(&settings.abs_path)
            .await
            .context("could not open the root of the working directory")?;

        let mut initial_snapshot = Snapshot {
            root_name,
            abs_path: Arc::clone(&settings.abs_path),
            tree_by_path: Default::default(),
            file_scan_exclusions: Arc::clone(&settings.monitoring_exclude_list),
        };

        if let Some(metadata) = root_metadata {
            initial_snapshot.tree_by_path.insert(
                settings.abs_path.to_path_buf(),
                WorktreeEntry {
                    kind: WorktreeEntryKind::PendingDir,
                    path: settings.abs_path.clone(),
                    modified: metadata.modified,
                    is_symlink: metadata.is_symlink,
                },
            );
        };

        let (share_tx, share_rx) = mpsc::unbounded_channel();

        let initial_state = {
            let state = LocalWorktreeState {
                prev_snapshot: None,
                last_snapshot: initial_snapshot.clone(),
            };

            Arc::new(RwLock::new(state))
        };

        Self::run_on_background(
            fs,
            share_tx,
            initial_state.clone(),
            Arc::clone(&settings.abs_path),
        )
        .await?;

        Ok(Self {
            state: initial_state,
            share_rx: Arc::new(Mutex::new(share_rx)),
        })
    }

    async fn run_on_background(
        fs: Arc<dyn FS>,
        share_tx: mpsc::UnboundedSender<WorktreeEvent>,
        initial_state: Arc<RwLock<LocalWorktreeState>>,
        abs_path: Arc<Path>,
    ) -> Result<()> {
        let (sync_tx, sync_rx) = mpsc::unbounded_channel();
        let initial_state_lock = initial_state.read().await;
        let initial_snapshot = initial_state_lock.last_snapshot.clone();
        drop(initial_state_lock);

        Self::run_background_scanner(fs, sync_tx, initial_snapshot, abs_path).await;
        Self::run_background_event_listener(sync_rx, share_tx, initial_state).await?;

        Ok(())
    }
    async fn run_background_scanner(
        fs: Arc<dyn FS>,
        sync_tx: mpsc::UnboundedSender<WorktreeEvent>,
        initial_snapshot: Snapshot,
        abs_path: Arc<Path>,
    ) {
        let fs_event_stream = fs.watch(&abs_path, Duration::from_secs(1)).await;

        task::spawn(async move {
            let scanner = LocalWorktreeScanner::new(fs, sync_tx, initial_snapshot);
            if let Err(e) = scanner.run(abs_path, fs_event_stream).await {
                error!("Error in worktree scanner: {e}");
            }
        });
    }

    async fn run_background_event_listener(
        mut sync_rx: mpsc::UnboundedReceiver<WorktreeEvent>,
        share_tx: mpsc::UnboundedSender<WorktreeEvent>,
        state: Arc<RwLock<LocalWorktreeState>>,
    ) -> Result<()> {
        task::spawn(async move {
            while let Some(event) = sync_rx.recv().await {
                let mut state_lock = state.write().await;

                state_lock.prev_snapshot = Some(state_lock.last_snapshot.clone());
                state_lock
                    .last_snapshot
                    .tree_by_path
                    .insert(event.entry.path.to_path_buf(), event.clone().entry);
                drop(state_lock);

                if let Err(e) = share_tx.send(event) {
                    error!("Failed to send worktree event: {e}");
                }
            }
        });

        Ok(())
    }

    pub async fn event_stream(&self) -> impl Stream<Item = WorktreeEvent> {
        let rx_clone = Arc::clone(&self.share_rx);

        async_stream::stream! {
            let mut rx = rx_clone.lock().await;
            while let Some(event) = rx.recv().await {
                yield event;
            }
        }
    }
}

#[derive(Debug)]
pub struct LocalWorktreeScannerState {
    snapshot: Snapshot,
}

#[derive(Debug)]
pub struct LocalWorktreeScanner {
    fs: Arc<dyn FS>,
    sync_tx: mpsc::UnboundedSender<WorktreeEvent>,
    state: Mutex<LocalWorktreeScannerState>,
}

impl LocalWorktreeScanner {
    fn new(
        fs: Arc<dyn FS>,
        sync_tx: mpsc::UnboundedSender<WorktreeEvent>,
        snapshot: Snapshot,
    ) -> LocalWorktreeScanner {
        Self {
            fs,
            sync_tx,
            state: Mutex::new(LocalWorktreeScannerState { snapshot }),
        }
    }

    async fn run(
        &self,
        root_abs_path: Arc<Path>,
        mut fs_event_stream: Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>>,
    ) -> Result<()> {
        let (scan_job_tx, scan_job_rx) = mpsc::unbounded_channel();
        {
            let state_lock = self.state.lock().await;
            if let Some(root_entry) = state_lock
                .snapshot
                .tree_by_path
                .get(&root_abs_path.to_path_buf())
            {
                self.enqueue_scan_dir(root_abs_path, &root_entry, &scan_job_tx)
                    .await?;
            }
        }
        drop(scan_job_tx);
        self.index_deep(scan_job_rx).await;

        loop {
            select_biased! {
                paths = fs_event_stream.next().fuse() => {
                    let Some(mut paths) = paths else { break };
                    while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_event_stream.next()) {
                        paths.extend(more_paths);
                    }

                  if paths.contains(&PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/.moss/settings.json")) {

                    self.sync_tx
                        .send(WorktreeEvent {
                            kind: WorkTreeEventKind::Modify,
                            entry: WorktreeEntry {
                                kind: WorktreeEntryKind::ReadyFile,
                                path: Arc::from(Path::new("/Users/g10z3r/Project/4rchr4y/moss/.moss/settings.json")),
                                modified:SystemTime::now(),
                                is_symlink: false,
                            },
                        })
                        .unwrap();

                    println!("Event: {:?}", paths);
                  }

                }
            }
        }

        Ok(())
    }

    async fn enqueue_scan_dir(
        &self,
        abs_path: Arc<Path>,
        entry: &WorktreeEntry,
        scan_job_tx: &mpsc::UnboundedSender<WorktreeScanJob>,
    ) -> Result<()> {
        scan_job_tx.clone().send(WorktreeScanJob {
            abs_path,
            path: entry.path.clone(),
            scan_queue: scan_job_tx.clone(),
        })?;

        Ok(())
    }

    async fn populate_dir(
        &self,
        parent_path: &Arc<Path>,
        entry_list: impl IntoIterator<Item = WorktreeEntry>,
    ) {
        let mut state_lock = self.state.lock().await;
        let mut parent_entry = if let Some(entry) = state_lock
            .snapshot
            .tree_by_path
            .get(&parent_path.to_path_buf())
        {
            entry.clone()
        } else {
            warn!("populating a directory {parent_path:?} that has been removed");
            return;
        };

        match parent_entry.kind {
            WorktreeEntryKind::PendingDir => parent_entry.kind = WorktreeEntryKind::ReadyDir,
            WorktreeEntryKind::ReadyDir => {}
            _ => return,
        }

        for entry in entry_list {
            state_lock
                .snapshot
                .tree_by_path
                .insert(entry.path.to_path_buf(), entry.clone());
        }

        self.sync_tx
            .send(WorktreeEvent {
                kind: WorkTreeEventKind::Discovery,
                entry: parent_entry,
            })
            .unwrap();

        info!("populated a directory {parent_path:?}");
    }

    async fn index_deep(&self, mut scan_jobs_rx: mpsc::UnboundedReceiver<WorktreeScanJob>) {
        loop {
            select_biased! {
                job_option = scan_jobs_rx.recv().fuse() => {
                    if let Some(job) = job_option {
                        if let Err(e) = self.index_dir(&job).await {
                            error!("failed to scan directory {:?}: {}", job.abs_path, e)
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    async fn index_dir(&self, job: &WorktreeScanJob) -> Result<()> {
        {
            let state_lock = self.state.lock().await;
            if state_lock
                .snapshot
                .is_path_excluded(&job.path.to_path_buf())
            {
                // TODO: might make sense to do logging here
                return Ok(());
            }

            drop(state_lock)
        }

        let mut planned_job_list: Vec<WorktreeScanJob> = Vec::new();
        let mut entry_list: Vec<WorktreeEntry> = Vec::new();

        let mut dir_stream = self.fs.read_dir(&job.path).await?;
        while let Some(child) = dir_stream.next().await {
            let child_abs_path: Arc<Path> = match child {
                Ok(child_path_buf) => child_path_buf.into(),
                Err(e) => {
                    error!("path processing error: {e}");
                    continue;
                }
            };

            let child_name = child_abs_path.file_name().unwrap(); // TODO: handle error
            let child_path: Arc<Path> = job.path.join(child_name).into();

            {
                let relative_path = job.path.join(child_name);
                let state_lock = self.state.lock().await;
                if state_lock.snapshot.is_path_excluded(&relative_path) {
                    debug!("unimplemented: skipping excluded child entry {relative_path:?}");
                    // TODO: call state.remove_path(&relative_path)
                    continue;
                }

                drop(state_lock);
            }

            let child_metadata = match self.fs.metadata(&child_abs_path).await {
                Ok(Some(metadata)) => metadata,
                Ok(None) => continue,
                Err(e) => {
                    error!("failed to process {child_abs_path:?}: {e:?}");
                    continue;
                }
            };

            let child_entry = WorktreeEntry::new(child_path.clone(), &child_metadata);

            if child_entry.is_dir() {
                planned_job_list.push(WorktreeScanJob {
                    abs_path: child_path,
                    path: child_abs_path,
                    scan_queue: job.scan_queue.clone(),
                })
            }

            entry_list.push(child_entry);
        }

        self.populate_dir(&job.path, entry_list).await;

        for j in planned_job_list {
            job.scan_queue.send(j).unwrap()
        }

        Ok(())
    }
}
