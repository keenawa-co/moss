use std::path::PathBuf;

pub struct Environment {
    pub untitled_workspaces_cache_dir: PathBuf, // $HOME/.config/untitled-workspaces
    pub workspaces_cache_dir: PathBuf,          // $HOME/.config/workspaces
    pub file_history_dir: PathBuf,              // $HOME/.config/history
    pub cache_dir: PathBuf,                     // $HOME/.config/cache

    // pub argv_resource: PathBuf, // $HOME/.config
    pub log_level: String,
    pub log_dir: PathBuf, // $HOME/.config/logs
}
