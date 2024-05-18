use std::time::SystemTime;

#[derive(Debug)]
pub struct Metadata {
    // pub inode: u64,
    pub modified: SystemTime,
    pub is_symlink: bool,
    pub is_dir: bool,
}
