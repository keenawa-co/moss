use anyhow::Context;
use jsonpath_lib as jsonpath;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    sync::Mutex,
};

#[async_trait::async_trait]
pub trait Settings {
    async fn get_fragment<T>(&self, key: &str) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync;

    async fn create_fragment<T>(&self, fragment: T) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync;

    async fn overwrite_fragment<T>(&self, key: &str, value: T) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync;

    async fn append_to_array<T>(&self, key: &str, append_list: &[T]) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync;

    async fn remove_from_array_fragment<T>(&self, key: &str, item: &T) -> anyhow::Result<()>
    where
        T: Serialize + Send + Sync;
}

#[derive(Debug)]
pub struct FileAdapter {
    file: Arc<Mutex<tokio::fs::File>>,
    cache: Arc<Mutex<Value>>,
}

impl FileAdapter {
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
        })
    }

    pub async fn write_by_path<T: Serialize + Send + Sync>(
        &self,
        path: &str,
        value: T,
    ) -> anyhow::Result<Value> {
        let serialized_value = serde_json::to_value(value).context("Failed to serialize value")?;
        let segments: Vec<&str> = path.trim_matches('/').split('/').collect();

        let mut cache_lock = self.cache.lock().await;
        let mut current = &mut *cache_lock;

        for part in &segments[..segments.len() - 1] {
            current = current
                .as_object_mut()
                .with_context(|| format!("Expected JSON object at '{}'", part))?
                .entry(part.to_string())
                .or_insert_with(|| json!({}));
        }

        let last_part = segments.last().expect("Path should not be empty");
        current
            .as_object_mut()
            .with_context(|| format!("Expected JSON object at '{}'", last_part))?
            .insert(last_part.to_string(), serialized_value);

        let new_content = cache_lock.clone();

        self.overwrite_file(serde_json::to_string_pretty(&new_content)?)
            .await
            .context("Failed to write to file")?;

        Ok(new_content)
    }

    pub async fn get_by_path<T: for<'de> Deserialize<'de>>(&self, path: &str) -> anyhow::Result<T> {
        let cache_lock = self.cache.lock().await;
        let fragment = cache_lock
            .pointer(path)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Key not found"))?;

        let module_settings = serde_json::from_value(fragment.clone())?;
        Ok(module_settings)
    }
}

impl FileAdapter {
    async fn overwrite_file(&self, content: String) -> anyhow::Result<()> {
        let mut file_lock = self.file.lock().await;

        file_lock.seek(tokio::io::SeekFrom::Start(0)).await?;
        file_lock.set_len(content.len() as u64).await?;
        file_lock.write_all(content.as_bytes()).await?;
        file_lock.flush().await?;

        Ok(())
    }
}
