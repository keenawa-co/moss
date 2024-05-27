use radix_trie::{Trie, TrieKey};
use std::{borrow::Borrow, path::Path, sync::Arc, time::SystemTime};

#[derive(Debug, Clone)]
pub struct LocalFiletree<T>(Trie<T, LocalFiletreeEntry>)
where
    T: Borrow<T>,
    T: TrieKey;

impl<T> LocalFiletree<T>
where
    T: Borrow<T>,
    T: TrieKey,
{
    pub fn get(&self, key: &T) -> Option<&LocalFiletreeEntry> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: T, value: LocalFiletreeEntry) -> Option<LocalFiletreeEntry> {
        self.0.insert(key, value)
    }
}

impl<T: TrieKey> Default for LocalFiletree<T> {
    fn default() -> Self {
        Self(Trie::new())
    }
}

impl<T: TrieKey> LocalFiletree<T> {
    pub fn new() -> Self {
        return Self(Trie::new());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalFiletreeEntryKind {
    PendingDir,
    ReadyDir,
    ReadyFile,
}

impl LocalFiletreeEntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(
            self,
            LocalFiletreeEntryKind::ReadyDir | LocalFiletreeEntryKind::PendingDir
        )
    }

    pub fn is_file(&self) -> bool {
        matches!(self, LocalFiletreeEntryKind::ReadyFile)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFiletreeEntry {
    pub kind: LocalFiletreeEntryKind,
    pub path: Arc<Path>,
    pub modified: SystemTime,
    pub is_symlink: bool,
}

impl LocalFiletreeEntry {
    pub fn new(path: Arc<Path>, metadata: &fs::file::Metadata) -> Self {
        Self {
            kind: if metadata.is_dir {
                LocalFiletreeEntryKind::PendingDir
            } else {
                LocalFiletreeEntryKind::ReadyFile
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
