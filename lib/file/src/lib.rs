#[cfg(feature = "graphql")]
use async_graphql::Object;

use std::{
    borrow::Cow,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[macro_use]
extern crate serde;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub mod_time: SystemTime,
    pub is_dir: bool,
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

#[cfg(feature = "graphql")]
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
