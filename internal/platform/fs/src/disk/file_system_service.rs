use anyhow::Result;
use futures::{AsyncRead, Stream};
use smol::stream::StreamExt;
use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
};

use crate::common::file_system_service::{AbstractFileSystemService, CreateOptions, Metadata};

#[async_trait]
pub trait AbstractDiskFileSystemService: AbstractFileSystemService {
    async fn truncate_file(&self, path: &PathBuf) -> Result<()>;
    async fn file_exists(&self, path: &PathBuf) -> bool;
    async fn metadata(&self, path: &PathBuf) -> Result<Option<Metadata>>;
}

// TODO: include LogService
#[derive(Debug)]
pub struct DiskFileSystemService {}

impl DiskFileSystemService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AbstractDiskFileSystemService for DiskFileSystemService {
    async fn truncate_file(&self, path: &PathBuf) -> Result<()> {
        let _file = smol::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .await?;

        Ok(())
    }

    async fn file_exists(&self, path: &PathBuf) -> bool {
        smol::fs::metadata(path).await.is_ok()
    }

    async fn metadata(&self, path: &PathBuf) -> Result<Option<Metadata>> {
        match smol::fs::symlink_metadata(path).await {
            Ok(symlink_metadata) => {
                let is_symlink = symlink_metadata.file_type().is_symlink();
                let metadata = if is_symlink {
                    smol::fs::metadata(path).await?
                } else {
                    symlink_metadata
                };

                Ok(Some(Metadata {
                    modified: metadata.modified()?,
                    is_symlink,
                    is_dir: metadata.is_dir(),
                }))
            }
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound | io::ErrorKind::Other
                    if err.raw_os_error() == Some(libc::ENOTDIR) =>
                {
                    Ok(None)
                }
                _ => Err(anyhow!(err)),
            },
        }
    }
}

#[async_trait]
impl AbstractFileSystemService for DiskFileSystemService {
    async fn create_dir(&self, path: &PathBuf) -> Result<()> {
        Ok(smol::fs::create_dir_all(path).await?)
    }

    async fn read_dir(
        &self,
        path: &PathBuf,
    ) -> Result<Pin<Box<dyn Send + Stream<Item = Result<PathBuf>>>>> {
        let result = smol::fs::read_dir(path).await?.map(|entry| match entry {
            Ok(entry) => Ok(entry.path()),
            Err(error) => Err(anyhow!("failed to read dir entry {:?}", error)),
        });

        Ok(Box::pin(result))
    }

    async fn create_file(&self, path: &PathBuf, options: CreateOptions) -> Result<()> {
        let mut open_options = smol::fs::OpenOptions::new();
        open_options.write(true).create(true);
        if options.overwrite {
            open_options.truncate(true);
        } else if !options.ignore_if_exists {
            open_options.create_new(true);
        }
        open_options.open(path).await?;
        Ok(())
    }

    async fn create_file_with_content(
        &self,
        path: &PathBuf,
        content: Pin<&mut (dyn AsyncRead + Send)>,
    ) -> Result<()> {
        let mut file = smol::fs::File::create(path).await?;
        futures::io::copy(content, &mut file).await?;
        Ok(())
    }

    // OPTIMIZE: read with cursor
    async fn read_file(&self, path: &PathBuf) -> Result<Box<dyn io::Read>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }

    async fn is_file(&self, path: &PathBuf) -> bool {
        smol::fs::metadata(path)
            .await
            .map_or(false, |metadata| metadata.is_file())
    }

    async fn is_dir(&self, path: &PathBuf) -> bool {
        smol::fs::metadata(path)
            .await
            .map_or(false, |metadata| metadata.is_dir())
    }
}
