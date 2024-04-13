#[cfg(feature = "gql")]
use async_graphql::Object;

use futures::Stream;
use smol::stream::StreamExt;
use std::{
    borrow::Cow,
    io,
    path::{Path, PathBuf},
    pin::Pin,
    time::{SystemTime, UNIX_EPOCH},
};

#[async_trait]
pub trait FS: Send + Sync {
    async fn read_dir(
        &self,
        path: &Path,
    ) -> anyhow::Result<Pin<Box<dyn Send + Stream<Item = anyhow::Result<PathBuf>>>>>;
    async fn read_file(&self, path: &Path) -> anyhow::Result<Box<dyn io::Read>>;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub mod_time: SystemTime,
    pub is_dir: bool,
}

#[cfg(feature = "gql")]
#[Object]
impl FileInfo {
    pub async fn path(&self) -> Cow<str> {
        self.path.to_string_lossy()
    }

    pub async fn size(&self) -> u64 {
        self.size
    }

    pub async fn mod_time(&self) -> async_graphql::Result<i64> {
        Ok(self
            .mod_time
            .duration_since(UNIX_EPOCH)
            .expect("File modification time is before UNIX EPOCH")
            .as_secs() as i64)
    }

    pub async fn is_dir(&self) -> bool {
        self.is_dir
    }
}

impl FileInfo {
    pub fn new(path_buf: PathBuf) -> anyhow::Result<Self> {
        let metadata = path_buf.metadata()?;

        Ok(Self {
            path: path_buf,
            size: metadata.len(),
            mod_time: metadata.modified()?,
            is_dir: metadata.is_dir(),
        })
    }
}

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl FS for FileSystem {
    async fn read_dir(
        &self,
        path: &Path,
    ) -> anyhow::Result<Pin<Box<dyn Send + Stream<Item = anyhow::Result<PathBuf>>>>> {
        let result = smol::fs::read_dir(path).await?.map(|entry| match entry {
            Ok(entry) => Ok(entry.path()),
            Err(error) => Err(anyhow!("failed to read dir entry {:?}", error)),
        });
        Ok(Box::pin(result))
    }

    async fn read_file(&self, path: &Path) -> anyhow::Result<Box<dyn io::Read>> {
        Ok(Box::new(std::fs::File::open(path)?))
    }
}
