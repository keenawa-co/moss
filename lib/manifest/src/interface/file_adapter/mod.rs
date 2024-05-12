mod ignore_list_storage_impl;

use fs::real::{self, FileSystem};
use smol::{
    fs::{File, OpenOptions},
    io::AsyncReadExt,
};
use std::{io, path::Path, sync::Arc};
use tokio::sync::Mutex;

use crate::model::file::RootFile;

use self::ignore_list_storage_impl::IgnoredListStorageImpl;

use super::file::{FileAdapter, IgnoredListStorage};

#[derive(Debug)]
pub struct FileJsonAdapter {
    ignored_list_storage: Arc<dyn IgnoredListStorage>,
}

impl FileJsonAdapter {
    pub async fn new(path: &Path) -> io::Result<Self> {
        // let realfs = real::FileSystem::new();
        // let file = realfs.open_with_options(path).await?;

        let mut file = OpenOptions::new().write(true).read(true).open(path).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        let state: RootFile = serde_json::from_str(&contents)?;
        let state_guard = Arc::new(Mutex::new(state));
        let file_guard = Mutex::new(file);

        let ignored_list_storage = IgnoredListStorageImpl::new(state_guard, file_guard);

        Ok(Self {
            ignored_list_storage: Arc::new(ignored_list_storage),
        })
    }
}

impl FileAdapter for FileJsonAdapter {
    fn ignored_list_storage(&self) -> Arc<dyn IgnoredListStorage> {
        Arc::clone(&self.ignored_list_storage)
    }
}
