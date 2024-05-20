use anyhow::Result;
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
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

use super::tree::{FileTree, Snapshot, WorktreeEntry};
use crate::worktree::tree::WorktreeEntryKind;

#[derive(Debug)]
pub struct TreeScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<TreeScanJob>,
}

#[derive(Debug, Serialize, Clone)]
pub enum WorkTreeEventKind {
    Create,
    Delete,
    Modify,
    Discovery,
}

#[derive(Debug, Clone)]
pub struct WorkTreeEvent {
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
    state: Mutex<LocalWorktreeState>,
    update_rx: Arc<Mutex<mpsc::UnboundedReceiver<WorkTreeEvent>>>,
}

fn default_file_scan_exclusions() -> HashSet<PathBuf> {
    let mut hs = HashSet::new();
    hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/view"));
    hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/target"));
    hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/.git"));

    hs
}

impl LocalWorktree {
    pub async fn new(fs: Arc<dyn FS>, abs_path: Arc<Path>) -> Arc<Self> {
        let root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());

        let root_metadata = fs
            .metadata(&Path::new("/Users/g10z3r/Project/4rchr4y/moss"))
            .await
            .unwrap() // TODO: handle option None
            .unwrap();

        let mut initial_filetree_by_path = FileTree::new();
        initial_filetree_by_path.insert(
            PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss"),
            WorktreeEntry {
                kind: WorktreeEntryKind::PendingDir,
                path: Arc::from(Path::new("/Users/g10z3r/Project/4rchr4y/moss")),
                modified: root_metadata.modified,
                is_symlink: root_metadata.is_symlink,
            },
        );

        let initial_snapshot = Snapshot {
            root_name,
            abs_path: abs_path.clone(),
            tree_by_path: initial_filetree_by_path,
            file_scan_exclusions: default_file_scan_exclusions(), // FIXME:
        };

        let (update_tx, update_rx) = mpsc::unbounded_channel();
        let (sync_tx, mut sync_rx) = mpsc::unbounded_channel();

        {
            let initial_snapshot_clone = initial_snapshot.clone();
            let fs_clone = fs.clone();
            let abs_path_clone = abs_path.clone();

            let fs_event_stream = fs.watch(&abs_path, Duration::from_secs(1)).await;
            task::spawn(async {
                LocalWorktreeScanner::new(fs_clone, sync_tx, initial_snapshot_clone)
                    .run(abs_path_clone, fs_event_stream)
                    .await
                    .unwrap();
            });
        }

        let initial_state = LocalWorktreeState {
            prev_snapshot: None,
            last_snapshot: initial_snapshot,
        };

        let worktree = Arc::new(Self {
            state: Mutex::new(initial_state),
            update_rx: Arc::new(Mutex::new(update_rx)),
        });

        {
            let worktree_clone = Arc::clone(&worktree);
            task::spawn(async move {
                while let Some(event) = sync_rx.recv().await {
                    dbg!(&event.entry.path.to_path_buf());

                    let mut state_lock = worktree_clone.state.lock().await;

                    state_lock.prev_snapshot = Some(state_lock.last_snapshot.clone());
                    state_lock
                        .last_snapshot
                        .tree_by_path
                        .insert(event.entry.path.to_path_buf(), event.clone().entry);

                    update_tx.send(event).unwrap();
                }
            });
        }

        worktree
    }

    pub async fn event_stream(&self) -> impl Stream<Item = WorkTreeEvent> {
        let rx_clone = Arc::clone(&self.update_rx);

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
    sync_tx: mpsc::UnboundedSender<WorkTreeEvent>,
    state: Mutex<LocalWorktreeScannerState>,
}

impl LocalWorktreeScanner {
    fn new(
        fs: Arc<dyn FS>,
        sync_tx: mpsc::UnboundedSender<WorkTreeEvent>,
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
                        .send(WorkTreeEvent {
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
        scan_job_tx: &mpsc::UnboundedSender<TreeScanJob>,
    ) -> Result<()> {
        scan_job_tx.clone().send(TreeScanJob {
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

            self.sync_tx
                .send(WorkTreeEvent {
                    kind: WorkTreeEventKind::Discovery,
                    entry,
                })
                .unwrap();
        }

        info!("populated a directory {parent_path:?}");
    }

    async fn index_deep(&self, mut scan_jobs_rx: mpsc::UnboundedReceiver<TreeScanJob>) {
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

    async fn index_dir(&self, job: &TreeScanJob) -> Result<()> {
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

        let mut planned_job_list: Vec<TreeScanJob> = Vec::new();
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
                planned_job_list.push(TreeScanJob {
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
