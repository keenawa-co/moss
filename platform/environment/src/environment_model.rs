use std::path::PathBuf;

pub struct Environment {
    pub untitled_workspaces_cache_dir: PathBuf, // $HOME/.config/moss/untitled-workspaces
    pub workspaces_cache_dir: PathBuf,          // $HOME/.config/moss/workspaces
    pub file_history_dir: PathBuf,              // $HOME/.config/moss/history
    pub cache_dir: PathBuf,                     // $HOME/.config/moss/cache

    // pub argv_resource: PathBuf, // $HOME/.config/moss
    pub log_level: String,
    pub log_dir: PathBuf, // $HOME/.config/moss/logs
}
