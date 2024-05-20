pub mod local;
pub mod tree;

use fs::FS;
use hashbrown::HashSet;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::{sync::mpsc, task};

use self::tree::{FileTree, WorktreeEntry, WorktreeEntryKind};
use self::{local::LocalWorktree, tree::Snapshot};

#[derive(Debug)]
pub enum Worktree {
    Local(Arc<LocalWorktree>),
}

impl Worktree {
    pub async fn local(fs: Arc<dyn FS>, abs_path: Arc<Path>) -> Self {
        // let root_name = abs_path
        //     .file_name()
        //     .map_or(String::new(), |f| f.to_string_lossy().to_string());

        // let mut ft = FileTree::new();

        // let ms = fs
        //     .metadata(&Path::new("/Users/g10z3r/Project/4rchr4y/moss"))
        //     .await
        //     .unwrap()
        //     .unwrap();

        // ft.insert(
        //     PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss"),
        //     Entry {
        //         kind: EntryKind::PendingDir,
        //         path: Arc::from(Path::new("/Users/g10z3r/Project/4rchr4y/moss")),
        //         modified: ms.modified,
        //         is_symlink: ms.is_symlink,
        //     },
        // );

        // let mut hs = HashSet::new();
        // hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/view"));
        // hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/target"));
        // hs.insert(PathBuf::from("/Users/g10z3r/Project/4rchr4y/moss/.git"));

        // let snap = Snapshot {
        //     abs_path: abs_path.clone(),
        //     root_name,
        //     tree_by_path: ft,
        //     file_scan_exclusions: hs,
        // };

        // {
        //     let (_scan_requests_tx, scan_requests_rx) = mpsc::unbounded_channel::<Arc<Path>>();
        //     let snap_clone = snap.clone();
        //     let fs_clone = fs.clone();
        //     let fs_event_stream = fs.watch(&abs_path, Duration::from_secs(1)).await;

        //     task::spawn(async {
        //         TreeScanner::new(fs_clone, scan_requests_rx, snap_clone)
        //             .run(fs_event_stream)
        //             .await;
        //     });
        // }

        // todo!()

        Worktree::Local(LocalWorktree::new(fs, abs_path).await)
    }
}
