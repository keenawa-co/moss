use async_graphql::Object;

use crate::worktree::local::event::{FileSystemEvent, ScannerEvent};

// TODO: implement as Interface
// https://async-graphql.github.io/async-graphql/en/define_interface.html#interface
#[derive(Debug)]
pub enum WorktreeEvent {
    FileSystem(FileSystemEvent),
    Scanner(ScannerEvent),
}

#[Object]
impl WorktreeEvent {
    pub async fn kind(&self) -> String {
        match self {
            WorktreeEvent::FileSystem(FileSystemEvent::Created(_)) => "created".to_string(),
            WorktreeEvent::FileSystem(FileSystemEvent::Deleted(_)) => "deleted".to_string(),
            WorktreeEvent::FileSystem(FileSystemEvent::Modified(_)) => "modified".to_string(),
            WorktreeEvent::Scanner(ScannerEvent::Discovered(_)) => "discovered".to_string(),
        }
    }

    pub async fn path(&self) -> Vec<String> {
        match self {
            WorktreeEvent::FileSystem(FileSystemEvent::Created(e))
            | WorktreeEvent::FileSystem(FileSystemEvent::Deleted(e))
            | WorktreeEvent::FileSystem(FileSystemEvent::Modified(e)) => {
                vec![e.path.to_string_lossy().to_string()]
            }

            WorktreeEvent::Scanner(ScannerEvent::Discovered(e)) => e
                .iter()
                .map(|item| item.path.to_string_lossy().to_string())
                .collect(),
        }
    }
}
