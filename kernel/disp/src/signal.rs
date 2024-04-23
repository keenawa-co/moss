use common::id::MNID;
use std::path::PathBuf;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum FileSignal {
    Modify(Vec<PathBuf>),
    Watch(PathBuf),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum SignalType {
    File(FileSignal),
}

#[derive(Debug, Clone)]
pub struct Signal {
    pub id: MNID,
    pub typ: SignalType,
}

impl Signal {
    pub fn new(origin: SignalType) -> Self {
        Self {
            id: MNID::new(),
            typ: origin,
        }
    }
}
