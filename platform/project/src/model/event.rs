use async_graphql::{Interface, Object};
use graphql_utl::path::Path as GraphQLPath;
use std::fmt::Debug;

use super::filetree::LocalFiletreeEntry;

#[derive(Debug)]
pub enum FileSystemEvent {
    Created(LocalFiletreeEntry),
    Deleted(LocalFiletreeEntry),
    Modified(LocalFiletreeEntry),
}

#[Object]
impl FileSystemEvent {
    async fn tag(&self) -> String {
        match self {
            FileSystemEvent::Created(_) => "created".to_string(),
            FileSystemEvent::Deleted(_) => "deleted".to_string(),
            FileSystemEvent::Modified(_) => "modified".to_string(),
        }
    }

    async fn path(&self) -> Vec<GraphQLPath> {
        match self {
            FileSystemEvent::Created(e)
            | FileSystemEvent::Deleted(e)
            | FileSystemEvent::Modified(e) => {
                vec![GraphQLPath::new(e.path.to_path_buf())]
            }
        }
    }
}

#[derive(Debug)]
pub enum ScannerEvent {
    Discovered(Vec<LocalFiletreeEntry>),
}

#[Object]
impl ScannerEvent {
    async fn tag(&self) -> String {
        match self {
            ScannerEvent::Discovered(_) => "discovered".to_string(),
        }
    }

    async fn path(&self) -> Vec<GraphQLPath> {
        match self {
            ScannerEvent::Discovered(e) => e
                .iter()
                .map(|item| GraphQLPath::new(item.path.to_path_buf()))
                .collect::<Vec<GraphQLPath>>(),
        }
    }
}

#[derive(Interface)]
#[graphql(
    field(name = "tag", ty = "String"),
    field(name = "path", ty = "Vec<GraphQLPath>")
)]
pub enum WorktreeEvent {
    FileSystem(FileSystemEvent),
    Scanner(ScannerEvent),
}

// #[Object]
// impl WorktreeEvent {
//     pub async fn kind(&self) -> String {
//         match self {
//             WorktreeEvent::FileSystem(FileSystemEvent::Created(_)) => "created".to_string(),
//             WorktreeEvent::FileSystem(FileSystemEvent::Deleted(_)) => "deleted".to_string(),
//             WorktreeEvent::FileSystem(FileSystemEvent::Modified(_)) => "modified".to_string(),
//             WorktreeEvent::Scanner(ScannerEvent::Discovered(_)) => "discovered".to_string(),
//         }
//     }

//     pub async fn path(&self) -> Vec<String> {
//         match self {
//             WorktreeEvent::FileSystem(FileSystemEvent::Created(e))
//             | WorktreeEvent::FileSystem(FileSystemEvent::Deleted(e))
//             | WorktreeEvent::FileSystem(FileSystemEvent::Modified(e)) => {
//                 vec![e.path.to_string_lossy().to_string()]
//             }

//             WorktreeEvent::Scanner(ScannerEvent::Discovered(e)) => e
//                 .iter()
//                 .map(|item| item.path.to_string_lossy().to_string())
//                 .collect(),
//         }
//     }
// }
