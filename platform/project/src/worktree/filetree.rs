use radix_trie::{Trie, TrieKey};
use std::{borrow::Borrow, path::Path, sync::Arc, time::SystemTime};

#[derive(Debug, Clone)]
pub struct FileTree<T>(Trie<T, FiletreeEntry>)
where
    T: Borrow<T>,
    T: TrieKey;

impl<T> FileTree<T>
where
    T: Borrow<T>,
    T: TrieKey,
{
    pub fn get(&self, key: &T) -> Option<&FiletreeEntry> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: T, value: FiletreeEntry) -> Option<FiletreeEntry> {
        self.0.insert(key, value)
    }
}

impl<T: TrieKey> Default for FileTree<T> {
    fn default() -> Self {
        Self(Trie::new())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileTreeEntryKind {
    PendingDir,
    ReadyDir,
    ReadyFile,
}

impl FileTreeEntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(
            self,
            FileTreeEntryKind::ReadyDir | FileTreeEntryKind::PendingDir
        )
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileTreeEntryKind::ReadyFile)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiletreeEntry {
    pub kind: FileTreeEntryKind,
    pub path: Arc<Path>,
    pub modified: SystemTime,
    pub is_symlink: bool,
}

impl FiletreeEntry {
    pub fn new(path: Arc<Path>, metadata: &fs::file::Metadata) -> Self {
        Self {
            kind: if metadata.is_dir {
                FileTreeEntryKind::PendingDir
            } else {
                FileTreeEntryKind::ReadyFile
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
