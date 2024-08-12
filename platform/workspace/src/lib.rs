use std::path::PathBuf;

pub struct Workspace {
    pub id: Option<String>,
    pub folders: Vec<String>,
    pub configuration_uri: Option<PathBuf>,
}
