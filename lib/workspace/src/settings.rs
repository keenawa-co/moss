use anyhow::{Context, Result};
use jsonpath_lib as jsonpath;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt},
    sync::Mutex,
};

#[derive(Debug)]
pub struct Settings {
    cache: Arc<Mutex<Value>>,
    file: Arc<Mutex<tokio::fs::File>>,
}

impl Settings {
    pub async fn new(file_path: &PathBuf) -> Result<Self> {
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
}

impl Settings {
    pub async fn get_by_key<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let cache_lock = self.cache.lock().await;
        let fragment = jsonpath::select(&cache_lock, key)?
            .get(0)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Key not found"))?;

        let module_settings = serde_json::from_value(fragment.clone())?;
        Ok(module_settings)
    }

    pub async fn overwrite_by_key<T: Serialize>(&self, key: &str, value: T) -> Result<()> {
        let serialized_value = serde_json::to_value(value)?;

        // Lock content and modify it
        let new_content = {
            let mut cache_lock = self.cache.lock().await;
            let result = jsonpath::replace_with(cache_lock.clone(), key, &mut |_| {
                Some(serialized_value.clone())
            })?;
            *cache_lock = result.clone();
            result
        };

        Ok(self
            .overwrite_file(serde_json::to_string_pretty(&new_content)?)
            .await?)
    }

    pub async fn append_to_array<T: Serialize>(
        &self,
        key: &str,
        append_list: &[T],
    ) -> anyhow::Result<()> {
        let serialized_value = serde_json::to_value(append_list)?;

        // Lock content and modify it
        let new_content = {
            let mut cache_lock = self.cache.lock().await;
            let result = jsonpath::replace_with(cache_lock.clone(), key, &mut |v| {
                if let Value::Array(mut array) = v {
                    serialized_value
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .for_each(|item| array.push(item.clone()));

                    Some(Value::Array(array.to_vec()))
                } else {
                    None
                }
            })?;
            *cache_lock = result.clone();
            result
        };

        Ok(self
            .overwrite_file(serde_json::to_string_pretty(&new_content)?)
            .await?)
    }
}

impl Settings {
    async fn overwrite_file(&self, content: String) -> anyhow::Result<()> {
        let mut file_lock = self.file.lock().await;

        file_lock.seek(tokio::io::SeekFrom::Start(0)).await?;
        file_lock.set_len(content.len() as u64).await?;
        file_lock.write_all(content.as_bytes()).await?;
        file_lock.flush().await?;

        Ok(())
    }
}
