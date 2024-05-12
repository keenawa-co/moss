use std::fmt::Debug;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};
use types::{id::NanoId, thing::Thing};

use crate::interface::file::FileAdapter;
use crate::interface::file_adapter::FileJsonAdapter;
use crate::interface::{
    cache::CacheAdapter,
    cache_adapter::{migration::CacheMigrator, CacheSQLiteAdapter},
};
use crate::model::ignored::IgnoredSource;

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
        let conn = seaorm_utl::conn::<CacheMigrator>(&conf.database_path).await?;

        Ok(Self {
            cache: Arc::new(CacheSQLiteAdapter::new(Arc::new(conn))),
            file: Arc::new(FileJsonAdapter::new(&Path::new(".moss/moss.json")).await?),
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

    pub async fn remove_from_ignore_list(&self, id: &NanoId) -> anyhow::Result<Option<Thing>> {
        let result = self.cache.ignored_list_repo().delete_by_id(id).await?;

        Ok(result)
    }

    pub async fn fetch_ignored_list(&self) -> anyhow::Result<Vec<IgnoredSource>> {
        let result = self.cache.ignored_list_repo().fetch_list().await?;

        Ok(result)
    }
}
