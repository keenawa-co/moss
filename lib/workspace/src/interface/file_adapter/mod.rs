mod ignore_list_storage_impl;

use std::{io, sync::Arc};
use tokio::sync::Mutex;

use self::ignore_list_storage_impl::IgnoredListStorageImpl;
use super::file::{FileAdapter, IgnoredListStorage};
use crate::model::file::ManifestFile;

#[derive(Debug)]
pub struct FileJsonAdapter {
    ignored_list_storage: Arc<dyn IgnoredListStorage>,
}

impl FileJsonAdapter {
    pub fn new(file: Arc<Mutex<smol::fs::File>>, state: Arc<Mutex<ManifestFile>>) -> Self {
        Self {
            ignored_list_storage: Arc::new(IgnoredListStorageImpl::new(state, file)),
        }
    }
}

impl FileAdapter for FileJsonAdapter {
    fn ignored_list_storage(&self) -> Arc<dyn IgnoredListStorage> {
        Arc::clone(&self.ignored_list_storage)
    }
}
