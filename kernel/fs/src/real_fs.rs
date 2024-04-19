use futures::Stream;
use smol::stream::StreamExt;
use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
};

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl super::FS for FileSystem {
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
