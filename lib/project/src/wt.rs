use anyhow::Result;
use fs::FS;
use futures::task::Poll;
use futures::{select_biased, AsyncRead, FutureExt, Stream};
use hashbrown::HashSet;
use radix_trie::Trie;
use smol::stream::StreamExt;
use std::{
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task;

pub struct WorktreeSettings {}

#[derive(Debug)]
pub enum Worktree {
    Local(LocalWorktree),
}

#[derive(Debug)]
pub struct LocalWorktree {
    snapshot: LocalSnapshot,
    scan_requests_tx: mpsc::UnboundedSender<Arc<Path>>,
    exclude_list: HashSet<String>,
}

impl Worktree {
    pub async fn local(fs: Arc<dyn FS>, abs_path: Arc<Path>) -> Self {
        // let scanner = TreeScanner { fs };

        let mut ft = FileTree::new();

        let ms = fs
            .metadata(&Path::new("/Users/g10z3r/Project/4rchr4y/moss"))
            .await
            .unwrap()
            .unwrap();

        ft.0.insert(
            PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss"),
            Entry {
                kind: EntryKind::PendingDir,
                path: Arc::from(Path::new("/Users/g10z3r/Project/4rchr4y/moss")),
                modified: ms.modified,
                is_symlink: ms.is_symlink,
            },
        );

        let mut hs = HashSet::new();
        hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/view"));
        hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/target"));
        hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/.git"));

        let (scan_requests_tx, scan_requests_rx) = mpsc::unbounded_channel::<Arc<Path>>();
        let root_name = abs_path
            .file_name()
            .map_or(String::new(), |f| f.to_string_lossy().to_string());

        scan_requests_tx.send(abs_path.clone()).unwrap();

        let snap = LocalSnapshot {
            abs_path: abs_path.clone(),
            root_name,
            tree_by_path: ft,
            file_scan_exclusions: hs,
        };

        // start background tasks
        {
            let snap_clone = snap.clone();
            let fs_clone = fs.clone();
            let fs_event_stream = fs.watch(&abs_path, Duration::from_secs(1)).await;

            task::spawn(async {
                TreeScanner::new(fs_clone, scan_requests_rx, snap_clone)
                    .run(fs_event_stream)
                    .await;
            });
        }

        Worktree::Local(LocalWorktree {
            snapshot: snap,
            scan_requests_tx,
            exclude_list: Default::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct FileTree<T>(Trie<T, Entry>)
where
    T: radix_trie::TrieKey;

impl<T: radix_trie::TrieKey> Default for FileTree<T> {
    fn default() -> Self {
        Self(Trie::new())
    }
}

impl<T: radix_trie::TrieKey> FileTree<T> {
    fn new() -> Self {
        return Self(Trie::new());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryKind {
    PendingDir,
    ReadyDir,
    ReadyFile,
}

impl EntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(self, EntryKind::ReadyDir | EntryKind::PendingDir)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, EntryKind::ReadyFile)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub kind: EntryKind,
    pub path: Arc<Path>,
    pub modified: SystemTime,
    pub is_symlink: bool,
}

impl Entry {
    fn new(path: Arc<Path>, metadata: &fs::file::Metadata) -> Self {
        Self {
            kind: if metadata.is_dir {
                EntryKind::PendingDir
            } else {
                EntryKind::ReadyFile
            },
            path,
            modified: metadata.modified,
            is_symlink: metadata.is_symlink,
        }
    }

    fn is_dir(&self) -> bool {
        self.kind.is_dir()
    }

    fn is_file(&self) -> bool {
        self.kind.is_file()
    }
}

#[derive(Debug, Clone)]
pub struct LocalSnapshot {
    abs_path: Arc<Path>,
    root_name: String,
    tree_by_path: FileTree<PathBuf>,
    file_scan_exclusions: HashSet<PathBuf>,
}

impl LocalSnapshot {
    fn is_path_excluded(&self, path: &PathBuf) -> bool {
        self.file_scan_exclusions.contains(path)
    }
}

#[derive(Debug)]
pub struct TreeScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<TreeScanJob>,
}

pub struct TreeScannerState {
    snapshot: LocalSnapshot,
    scanned_dirs: HashSet<String>,
}

impl TreeScannerState {
    async fn enqueue_scan_dir(
        &self,
        abs_path: Arc<Path>,
        entry: &Entry,
        scan_job_tx: &mpsc::UnboundedSender<TreeScanJob>,
    ) {
        let path = entry.path.clone();

        scan_job_tx
            .clone()
            .send(TreeScanJob {
                abs_path,
                path,
                scan_queue: scan_job_tx.clone(),
            })
            .unwrap()
    }

    fn populate_dir(
        &mut self,
        parent_path: &Arc<Path>,
        entry_list: impl IntoIterator<Item = Entry>,
    ) {
        let mut parent_entry = if let Some(entry) = self
            .snapshot
            .tree_by_path
            .0
            .get::<PathBuf>(&parent_path.to_path_buf())
        {
            entry.clone()
        } else {
            warn!("populating a directory {parent_path:?} that has been removed");
            return;
        };

        match parent_entry.kind {
            EntryKind::PendingDir => parent_entry.kind = EntryKind::ReadyDir,
            EntryKind::ReadyDir => {}
            _ => return,
        }

        for entry in entry_list {
            self.snapshot
                .tree_by_path
                .0
                .insert(entry.path.to_path_buf(), entry);
        }

        info!("populated a directory {parent_path:?}");
    }
}

pub struct TreeScanner {
    fs: Arc<dyn FS>,
    scan_requests_rx: mpsc::UnboundedReceiver<Arc<Path>>,
    state: Mutex<TreeScannerState>,
}

impl TreeScanner {
    pub fn new(
        fs: Arc<dyn FS>,
        scan_requests_rx: mpsc::UnboundedReceiver<Arc<Path>>,
        snapshot: LocalSnapshot,
    ) -> Self {
        Self {
            fs,
            scan_requests_rx,
            state: Mutex::new(TreeScannerState {
                snapshot,
                scanned_dirs: Default::default(),
            }),
        }
    }

    async fn run(&mut self, mut fs_event_stream: Pin<Box<dyn Send + Stream<Item = Vec<PathBuf>>>>) {
        let root_abs_path = self.state.lock().await.snapshot.abs_path.clone();

        let (scan_job_tx, scan_job_rx) = mpsc::unbounded_channel();

        {
            let state_lock = self.state.lock().await;
            if let Some(root_entry) = state_lock
                .snapshot
                .tree_by_path
                .0
                .get::<PathBuf>(&root_abs_path.to_path_buf())
            {
                state_lock
                    .enqueue_scan_dir(root_abs_path, &root_entry, &scan_job_tx)
                    .await;
            }
        }

        drop(scan_job_tx);
        self.scan_dirs(scan_job_rx).await;

        loop {
            select_biased! {
                request_option = self.scan_requests_rx.recv().fuse() => {
                    if let Some(request) = request_option {
                        if !self.process_scan_request(request).await {
                            return;
                        }
                    }
                }

                paths = fs_event_stream.next().fuse() => {
                    let Some(mut paths) = paths else { break };
                    while let Poll::Ready(Some(more_paths)) = futures::poll!(fs_event_stream.next()) {
                        paths.extend(more_paths);
                    }

                    // println!("Event: {:?}", paths);
                  if paths.contains(&PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/.moss/settings.json")) {
                    println!("Event: {:?}", paths);
                  }

                }
            }
        }
    }

    // deep_scan
    async fn scan_dirs(&mut self, mut scan_jobs_rx: mpsc::UnboundedReceiver<TreeScanJob>) {
        loop {
            select_biased! {
                // request_option = self.scan_requests_rx.recv().fuse() => {
                //     if let Some(request) = request_option {
                //         if !self.process_scan_request(request).await {
                //             return;
                //         }

                //     } else {
                //         break;
                //     }

                    // if !self.process_scan_request(request, true).await {
                    //     return;
                    // }
                // }

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

    async fn process_scan_request(&self, req: Arc<Path>) -> bool {
        info!("rescanning paths {:?}", req);

        true
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
        let mut entry_list: Vec<Entry> = Vec::new();

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

            let child_entry = Entry::new(child_path.clone(), &child_metadata);

            if child_entry.is_dir() {
                planned_job_list.push(TreeScanJob {
                    abs_path: child_path,
                    path: child_abs_path,
                    scan_queue: job.scan_queue.clone(),
                })
            }

            entry_list.push(child_entry);
        }

        let mut state_lock = self.state.lock().await;
        state_lock.populate_dir(&job.path, entry_list);

        for j in planned_job_list {
            job.scan_queue.send(j).unwrap()
        }

        Ok(())
    }
}
