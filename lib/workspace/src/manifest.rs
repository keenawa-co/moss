use sha2::Digest;
use sha2::Sha256;
use smol::{fs::OpenOptions, io::AsyncReadExt};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use types::{id::NanoId, thing::Thing};

use crate::interface::{
    cache::CacheAdapter,
    cache_adapter::{migration::CacheMigrator, CacheSQLiteAdapter},
};
use crate::model::ignored::IgnoredSource;
use crate::{
    interface::{file::FileAdapter, file_adapter::FileJsonAdapter},
    model::file::ManifestFile,
};

#[derive(Debug)]
pub struct Manifest {
    cache: Arc<dyn CacheAdapter>,
    file: Arc<dyn FileAdapter>,
}

pub struct Config {
    pub database_path: PathBuf,
}

impl Manifest {
    pub async fn new(conf: &Config) -> anyhow::Result<Self> {
        let mut file: smol::fs::File = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&Path::new(".moss/moss.json"))
            .await?;

        let content = {
            let mut result = String::new();
            file.read_to_string(&mut result).await?;
            result
        };

        let content_hash = {
            let mut hasher = Sha256::new();
            hasher.update(&content);
            format!("{:x}", hasher.finalize())
        };

        let state: ManifestFile = serde_json::from_str(&content)?;
        let state_guard = Arc::new(Mutex::new(state));
        let file_guard = Arc::new(Mutex::new(file));

        let conn = seaorm_utl::conn::<CacheMigrator>(&conf.database_path).await?;

        // TODO: Synchronize with the cache if the file amount differs from what was saved in the database

        dbg!(content_hash);

        let cache_adapter = CacheSQLiteAdapter::new(Arc::new(conn));
        let file_adapter = FileJsonAdapter::new(file_guard, state_guard);

        Ok(Self {
            cache: Arc::new(cache_adapter),
            file: Arc::new(file_adapter),
        })
    }
}

impl Manifest {
    pub async fn append_to_ignored_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> anyhow::Result<Vec<IgnoredSource>> {
        let _ = self
            .file
            .ignored_list_storage()
            .create_from_list(input_list)
            .await?;

        let result = self
            .cache
            .ignored_list_repo()
            .create_from_list(input_list)
            .await?;

        Ok(result)
    }

    pub async fn remove_from_ignore_list(
        &self,
        id: &NanoId,
    ) -> anyhow::Result<Option<Thing<NanoId>>> {
        let result = self.cache.ignored_list_repo().delete_by_id(id).await?;

        Ok(result)
    }

    pub async fn fetch_ignored_list(&self) -> anyhow::Result<Vec<IgnoredSource>> {
        let result = self.cache.ignored_list_repo().fetch_list().await?;

        Ok(result)
    }
}
