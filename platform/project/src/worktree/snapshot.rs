use hashbrown::HashSet;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use super::filetree::FileTree;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub root_name: String,
    pub abs_path: Arc<Path>,
    pub tree_by_path: FileTree<PathBuf>,
    pub file_scan_exclusions: Arc<HashSet<PathBuf>>,
}

impl Snapshot {
    pub fn is_path_excluded(&self, path: &PathBuf) -> bool {
        self.file_scan_exclusions.contains(path)
    }
}
