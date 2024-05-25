use super::tree::LocalWorktreeEntry;

#[derive(Debug)]
pub enum FileSystemEvent {
    Created(LocalWorktreeEntry),
    Deleted(LocalWorktreeEntry),
    Modified(LocalWorktreeEntry),
}

#[derive(Debug)]
pub enum ScannerEvent {
    Discovered(Vec<LocalWorktreeEntry>),
}
