#[cfg(feature = "graphql")]
use async_graphql::Object;

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub path: PathBuf,
    pub size: u64,
    pub mod_time: SystemTime,
    pub is_dir: bool,
}

impl Metadata {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        let metadata = path.metadata()?;

        Ok(Self {
            path: path.to_owned(),
            size: metadata.len(),
            mod_time: metadata.modified()?,
            is_dir: metadata.is_dir(),
        })
    }
}

#[cfg(feature = "graphql")]
#[Object]
impl Metadata {
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

pub mod json_file {
    use anyhow::Context;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::{path::PathBuf, sync::Arc};
    use tokio::{
        fs::OpenOptions,
        io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
        sync::Mutex,
    };

    use crate::file::Metadata;

    #[derive(Debug)]

    pub struct JsonFile {
        pub(self) file: Arc<Mutex<tokio::fs::File>>,
        pub(self) cache: Arc<Mutex<Value>>,
        pub metadata: Metadata,
    }

    impl JsonFile {
        pub async fn new(file_path: &PathBuf) -> anyhow::Result<Self> {
            let mut file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open(file_path)
                .await
                .with_context(|| format!("Failed to open or create file at {:?}", file_path))?;

            let content = {
                let mut result = String::new();
                file.read_to_string(&mut result).await?;
                if result.trim().is_empty() {
                    "{}".to_string() // Handle empty file case
                } else {
                    result
                }
            };

            Ok(Self {
                cache: Arc::new(Mutex::new(serde_json::from_str(&content)?)),
                file: Arc::new(Mutex::new(file)),
                metadata: Metadata::new(file_path)?,
            })
        }
    }

    impl JsonFile {
        pub const ROOT_PATH: &'static str = "/";

        pub async fn write_by_path<T: Serialize + Send + Sync>(
            &self,
            path: &str,
            value: T,
        ) -> anyhow::Result<Value> {
            let serialized_value =
                serde_json::to_value(value).context("Failed to serialize value")?;
            let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

            let mut cache_lock = self.cache.lock().await;
            let mut current = &mut *cache_lock;

            for part in &segments[..segments.len() - 1] {
                current = current
                    .as_object_mut()
                    .with_context(|| format!("Expected JSON object at {}", quote!(part)))?
                    .entry(part.to_string())
                    .or_insert_with(|| json!({}));
            }

            let last_part = segments.last().expect("Path should not be empty");
            current
                .as_object_mut()
                .with_context(|| format!("Expected JSON object at {}", quote!(last_part)))?
                .insert(last_part.to_string(), serialized_value);

            let new_content = cache_lock.clone();

            self.overwrite_file(serde_json::to_string_pretty(&new_content)?)
                .await
                .context("Failed to write to file")?;

            Ok(new_content)
        }

        pub async fn get_by_path<T: for<'de> Deserialize<'de>>(
            &self,
            path: &str,
        ) -> anyhow::Result<Option<T>> {
            let cache_lock = self.cache.lock().await;
            let fragment_value = if path == Self::ROOT_PATH {
                cache_lock.clone()
            } else {
                match cache_lock.pointer(path).cloned() {
                    Some(value) => value,
                    None => return Ok(None),
                }
            };

            match serde_json::from_value(fragment_value) {
                Ok(fragment_type) => Ok(Some(fragment_type)),
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        }

        async fn overwrite_file(&self, content: String) -> anyhow::Result<()> {
            let mut file_lock = self.file.lock().await;

            file_lock.seek(tokio::io::SeekFrom::Start(0)).await?;
            file_lock.set_len(content.len() as u64).await?;
            file_lock.write_all(content.as_bytes()).await?;
            file_lock.flush().await?;

            Ok(())
        }
    }
}
