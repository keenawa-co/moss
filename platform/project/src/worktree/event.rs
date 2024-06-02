use super::filetree::FiletreeEntry;

#[derive(Debug)]
pub enum WorktreeEvent {
    Created(Vec<FiletreeEntry>),
    Modified(Vec<FiletreeEntry>),
    Deleted(Vec<FiletreeEntry>),
}
