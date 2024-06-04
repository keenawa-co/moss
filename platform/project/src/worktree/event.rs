use app::context::Event;

use super::filetree::FiletreeEntry;

#[derive(Debug)]
pub enum WorktreeEvent {
    Created(Vec<FiletreeEntry>),
    Modified(Vec<FiletreeEntry>),
    Deleted(Vec<FiletreeEntry>),
}

unsafe impl Event for WorktreeEvent {
    const TYPE_NAME: &'static str = "WorktreeEvent";
}
