use anyhow::Result;
use app::context::{AppContext, AsyncAppContext};
use app::context_compact::AppContextCompact;
use fs::FS;
use futures::{future, task::Poll};
use futures::{select_biased, FutureExt, Stream};
use smol::{channel::Sender as SmolSender, stream::StreamExt};
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex};

use super::event::WorktreeEvent;
use super::filetree::{FileTreeEntryKind, FiletreeEntry};
use super::snapshot::Snapshot;

#[derive(Debug)]
struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

#[derive(Debug)]
struct ServiceState {
    snapshot: Snapshot,
}

#[derive(Debug)]
pub struct FileSystemScanService {
    fs: Arc<dyn FS>,
    sync_tx: SmolSender<WorktreeEvent>,
    state: Mutex<ServiceState>,
}

impl FileSystemScanService {
    pub fn new(
        fs: Arc<dyn FS>,
        sync_tx: SmolSender<WorktreeEvent>,
        snapshot: Snapshot,
    ) -> FileSystemScanService {
        Self {
            fs,
            sync_tx,
            state: Mutex::new(ServiceState { snapshot }),
        }
    }

    pub async fn run(
        &self,
        ctx: AppContextCompact,
        root_abs_path: Arc<Path>,
        mut fs_event_stream: Pin<Box<dyn Send + Stream<Item = notify::Event>>>,
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

        let tm1 = chrono::Utc::now();
        self.index_deep(&ctx, scan_job_rx).await;
        let tm2 = chrono::Utc::now();

        let t = tm2 - tm1;
        dbg!(t);

        // NOTE: use select_biased! to prioritize event queue

        loop {
            if let Some(notify_event) = fs_event_stream.next().await {
                self.handle_notify_event(notify_event).await.unwrap(); // TODO: handle error
            } else {
                break;
            }
        }

        Ok(())
    }

    async fn handle_notify_event(&self, event: notify::Event) -> Result<()> {
        let map_to_filetree_entry = |paths: Vec<PathBuf>, kind: FileTreeEntryKind| {
            paths
                .into_iter()
                .map(|item| FiletreeEntry {
                    kind,
                    path: Arc::from(item),
                    modified: None,
                    is_symlink: None,
                })
                .collect::<Vec<FiletreeEntry>>()
        };

        match event.kind {
            notify::EventKind::Create(kind) => {
                self.sync_tx
                    .try_send(WorktreeEvent::Created(map_to_filetree_entry(
                        event.paths,
                        FileTreeEntryKind::from(kind),
                    )))?;

                Ok(())
            }

            notify::EventKind::Modify(_) => {
                println!("Modify event: {:?}", event);
                Ok(())
            }

            notify::EventKind::Remove(kind) => {
                self.sync_tx
                    .try_send(WorktreeEvent::Created(map_to_filetree_entry(
                        event.paths,
                        FileTreeEntryKind::from(kind),
                    )))?;

                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn enqueue_scan_dir(
        &self,
        abs_path: Arc<Path>,
        entry: &FiletreeEntry,
        scan_job_tx: &mpsc::UnboundedSender<ScanJob>,
    ) -> Result<()> {
        scan_job_tx.clone().send(ScanJob {
            abs_path,
            path: entry.path.clone(),
            scan_queue: scan_job_tx.clone(),
        })?;

        Ok(())
    }

    async fn populate_dir(
        &self,
        ctx: &AppContextCompact,
        parent_path: &Arc<Path>,
        entry_list: Vec<FiletreeEntry>,
    ) {
        let mut state_lock = self.state.lock().await;
        let parent_entry = if let Some(entry) = state_lock
            .snapshot
            .tree_by_path
            .get(&parent_path.to_path_buf())
        {
            entry.clone()
        } else {
            warn!("populating a directory {parent_path:?} that has been removed");
            return;
        };

        if parent_entry.kind != FileTreeEntryKind::Dir {
            return;
        }

        for entry in &entry_list {
            state_lock
                .snapshot
                .tree_by_path
                .insert(entry.path.to_path_buf(), entry.clone());
        }

        // let mut r = ctx.clone();

        // r.with_event_registry_mut(|registry| {
        //     registry.dispatch_event(WorktreeEvent::Created(entry_list))
        // });

        // if let Err(e) = self.sync_tx.send(WorktreeEvent::Created(entry_list)).await {
        //     error!("Failed to send event: {e}");
        // }

        info!("populated a directory {parent_path:?}");
    }

    async fn index_deep(
        &self,
        ctx: &AppContextCompact,
        mut scan_jobs_rx: mpsc::UnboundedReceiver<ScanJob>,
    ) {
        loop {
            select_biased! {
                job_option = scan_jobs_rx.recv().fuse() => {
                    if let Some(job) = job_option {
                        if let Err(e) = self.index_dir(ctx, &job).await {
                            error!("failed to scan directory {:?}: {}", job.abs_path, e)
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    async fn index_dir(&self, ctx: &AppContextCompact, job: &ScanJob) -> Result<()> {
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

        let mut planned_job_list: Vec<ScanJob> = Vec::new();
        let mut entry_list: Vec<FiletreeEntry> = Vec::new();

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

            let child_entry = FiletreeEntry::new(child_path.clone(), &child_metadata);

            if child_entry.is_dir() {
                planned_job_list.push(ScanJob {
                    abs_path: child_path,
                    path: child_abs_path,
                    scan_queue: job.scan_queue.clone(),
                })
            }

            entry_list.push(child_entry);
        }

        self.populate_dir(ctx, &job.path, entry_list).await;

        for j in planned_job_list {
            job.scan_queue.send(j).unwrap()
        }

        Ok(())
    }
}
