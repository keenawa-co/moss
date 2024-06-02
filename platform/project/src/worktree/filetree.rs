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
    Any,
    File,
    Dir,
    Other,
}

impl From<notify::event::CreateKind> for FileTreeEntryKind {
    fn from(value: notify::event::CreateKind) -> Self {
        match value {
            notify::event::CreateKind::File => Self::File,
            notify::event::CreateKind::Folder => Self::Dir,
            notify::event::CreateKind::Other => Self::Other,
            notify::event::CreateKind::Any => Self::Any,
        }
    }
}

impl From<notify::event::RemoveKind> for FileTreeEntryKind {
    fn from(value: notify::event::RemoveKind) -> Self {
        match value {
            notify::event::RemoveKind::File => Self::File,
            notify::event::RemoveKind::Folder => Self::Dir,
            notify::event::RemoveKind::Other => Self::Other,
            notify::event::RemoveKind::Any => Self::Any,
        }
    }
}

impl FileTreeEntryKind {
    pub fn is_dir(&self) -> bool {
        matches!(self, FileTreeEntryKind::Dir)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, FileTreeEntryKind::File)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiletreeEntry {
    pub kind: FileTreeEntryKind,
    pub path: Arc<Path>,
    pub modified: Option<SystemTime>,
    pub is_symlink: Option<bool>,
}

impl FiletreeEntry {
    pub fn new(path: Arc<Path>, metadata: &fs::file::Metadata) -> Self {
        Self {
            kind: if metadata.is_dir {
                FileTreeEntryKind::Dir
            } else {
                FileTreeEntryKind::File
            },
            path,
            modified: Some(metadata.modified),
            is_symlink: Some(metadata.is_symlink),
        }
    }

    pub fn is_dir(&self) -> bool {
        self.kind.is_dir()
    }

    pub fn is_file(&self) -> bool {
        self.kind.is_file()
    }
}
