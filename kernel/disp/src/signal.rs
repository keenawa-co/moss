use common::id::MNID;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum FileSignal {
    Modify(Vec<PathBuf>),
}

#[derive(Debug, Clone)]
pub enum Origin {
    FileWatcher(FileSignal),
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub id: MNID,
    pub origin: Origin,
}

impl Signal {
    pub fn new(origin: Origin) -> Self {
        Self {
            id: MNID::new(),
            origin,
        }
    }
}
