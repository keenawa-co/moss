use anyhow::Result;
use futures::{AsyncRead, Stream};
use std::{
    fmt::Debug,
    io,
    path::{Path, PathBuf},
    pin::Pin,
    time::SystemTime,
};

#[derive(Copy, Clone)]
pub struct CreateOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

impl Default for CreateOptions {
    fn default() -> Self {
        Self {
            overwrite: false,
            ignore_if_exists: true,
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub modified: SystemTime,
    pub is_symlink: bool,
    pub is_dir: bool,
}

#[async_trait]
pub trait AbstractFileSystemService: Debug + Send + Sync {
    async fn create_dir(&self, path: &PathBuf) -> Result<()>;
    async fn read_dir(
        &self,
        path: &PathBuf,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>>;

    async fn create_file(&self, path: &PathBuf, options: CreateOptions) -> Result<()>;
    async fn create_file_with_content(
        &self,
        path: &PathBuf,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()>;

    async fn read_file(&self, path: &PathBuf) -> Result<Box<dyn io::Read>>;

    async fn is_file(&self, path: &PathBuf) -> bool;
    async fn is_dir(&self, path: &PathBuf) -> bool;
}
