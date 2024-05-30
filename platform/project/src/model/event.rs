use serde::{Serialize, Serializer};
use std::{path::Path, sync::Arc};

#[derive(Debug, Serialize)]
pub struct SharedWorktreeEntry {
    pub path: Arc<Path>,
    pub is_dir: bool,
}

#[derive(Debug)]
pub enum SharedWorktreeEvent {
    Created(Vec<SharedWorktreeEntry>),
    Deleted(Vec<SharedWorktreeEntry>),
    Modified(Vec<SharedWorktreeEntry>),
}

impl Serialize for SharedWorktreeEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SharedWorktreeEvent::Created(entries) => entries.serialize(serializer),
            SharedWorktreeEvent::Deleted(entries) => entries.serialize(serializer),
            SharedWorktreeEvent::Modified(entries) => entries.serialize(serializer),
        }
    }
}

#[derive(Debug)]
pub enum SharedEvent {
    WorktreeEvent(SharedWorktreeEvent),
}

impl Serialize for SharedEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SharedEvent::WorktreeEvent(event) => event.serialize(serializer),
        }
    }
}
