use serde::Serialize;
use std::{path::Path, sync::Arc};

#[derive(Debug, Serialize)]
pub struct SharedWorktreeEntry {
    pub path: Arc<Path>,
    pub is_dir: bool,
}

#[derive(Debug, Serialize)]
pub enum SharedWorktreeEvent {
    #[serde(rename = "created")]
    Created(Vec<SharedWorktreeEntry>),
    #[serde(rename = "modified")]
    Modified(Vec<SharedWorktreeEntry>),
    #[serde(rename = "deleted")]
    Deleted(Vec<SharedWorktreeEntry>),
}

// impl Serialize for SharedWorktreeEvent {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             SharedWorktreeEvent::Created(entries) => entries.serialize(serializer),
//             SharedWorktreeEvent::Deleted(entries) => entries.serialize(serializer),
//             SharedWorktreeEvent::Modified(entries) => entries.serialize(serializer),
//         }
//     }
// }
