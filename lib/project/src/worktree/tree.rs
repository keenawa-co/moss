use hashbrown::HashSet;
use radix_trie::{Trie, TrieKey};
use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub root_name: String,
    pub abs_path: Arc<Path>,
    pub tree_by_path: FileTree<PathBuf>,
    pub file_scan_exclusions: HashSet<PathBuf>,
}

impl Snapshot {
    pub fn is_path_excluded(&self, path: &PathBuf) -> bool {
        self.file_scan_exclusions.contains(path)
    }
}

#[derive(Debug, Clone)]
pub struct FileTree<T>(Trie<T, WorktreeEntry>)
where
    T: Borrow<T>,
    T: TrieKey;

impl<T> FileTree<T>
where
    T: Borrow<T>,
    T: TrieKey,
{
    pub fn get(&self, key: &T) -> Option<&WorktreeEntry> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: T, value: WorktreeEntry) -> Option<WorktreeEntry> {
        self.0.insert(key, value)
    }
}

impl<T: TrieKey> Default for FileTree<T> {
    fn default() -> Self {
        Self(Trie::new())
    }
}

impl<T: TrieKey> FileTree<T> {
    pub fn new() -> Self {
        return Self(Trie::new());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorktreeEntryKind {
    PendingDir,
    ReadyDir,
    ReadyFile,
}

impl WorktreeEntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(
            self,
            WorktreeEntryKind::ReadyDir | WorktreeEntryKind::PendingDir
        )
    }

    pub fn is_file(&self) -> bool {
        matches!(self, WorktreeEntryKind::ReadyFile)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorktreeEntry {
    pub kind: WorktreeEntryKind,
    pub path: Arc<Path>,
    pub modified: SystemTime,
    pub is_symlink: bool,
}

impl WorktreeEntry {
    pub fn new(path: Arc<Path>, metadata: &fs::file::Metadata) -> Self {
        Self {
            kind: if metadata.is_dir {
                WorktreeEntryKind::PendingDir
            } else {
                WorktreeEntryKind::ReadyFile
            },
            path,
            modified: metadata.modified,
            is_symlink: metadata.is_symlink,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.kind.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.kind.is_file()
    }
}
