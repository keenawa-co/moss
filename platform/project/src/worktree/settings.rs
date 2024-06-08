use hashbrown::HashSet;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct LocalWorktreeSettings {
    pub abs_path: Arc<Path>,
    pub monitoring_exclude_list: Arc<HashSet<PathBuf>>,
    pub watch_gitignore_entries: bool,
    pub auto_watch_new_entries: bool,
}
