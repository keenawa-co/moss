use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum WorkspaceId {
    Empty,
    Some(String),
}

#[derive(Debug)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub folders: Vec<String>,
    pub configuration_uri: Option<PathBuf>,
}
