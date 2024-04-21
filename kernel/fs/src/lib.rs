pub mod fw;
pub mod real_fs;

use futures::Stream;
use std::{
    io,
    path::{Path, PathBuf},
    pin::Pin,
};

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate anyhow;

#[async_trait]
pub trait FS: Send + Sync {
    async fn read_dir(
        &self,
        path: &Path,
    ) -> anyhow::Result<Pin<Box<dyn Send + Stream<Item = anyhow::Result<PathBuf>>>>>;
    async fn read_file(&self, path: &Path) -> anyhow::Result<Box<dyn io::Read>>;
}
