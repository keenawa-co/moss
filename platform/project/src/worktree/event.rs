use super::filetree::FiletreeEntry;

pub enum WorktreeEvent {
    FileSystem(FileSystemEvent),
    Scanner(ScannerEvent),
}

#[derive(Debug)]
pub enum FileSystemEvent {
    Created(FiletreeEntry),
    Deleted(FiletreeEntry),
    Modified(FiletreeEntry),
}

#[derive(Debug)]
pub enum ScannerEvent {
    Discovered(Vec<FiletreeEntry>),
}
