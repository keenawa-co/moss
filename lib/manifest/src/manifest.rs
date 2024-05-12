use common::id::NanoId;
use common::thing::Thing;
use hashbrown::HashSet;

use std::fmt::Debug;
use std::{path::PathBuf, sync::Arc};

use crate::interface::{
    cache::CacheAdapter,
    cache_adapter::{migration::CacheMigrator, CacheSQLiteAdapter},
};
use crate::model::ignored::IgnoredSource;

#[derive(Debug)]
pub struct Manifest {
    cache: Arc<dyn CacheAdapter>,
}

pub struct Config {
    pub database_path: PathBuf,
}

impl Manifest {
    pub async fn new(conf: &Config) -> anyhow::Result<Self> {
        let conn = dbutl::sqlite::conn::<CacheMigrator>(&conf.database_path).await?;

        Ok(Self {
            cache: Arc::new(CacheSQLiteAdapter::new(Arc::new(conn))),
        })
    }
}

impl Manifest {
    pub async fn append_to_ignored_list(
        &self,
        input_list: &Vec<PathBuf>,
    ) -> anyhow::Result<Vec<IgnoredSource>> {
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

    pub async fn fetch_ignored_list(&self) -> Arc<HashSet<IgnoredSource>> {
        let mut ignored_paths = HashSet::new();
        ignored_paths.insert(IgnoredSource {
            id: NanoId::new(),
            source: "/Users/g10z3r/Project/4rchr4y/moss/target/".to_string(),
        });

        Arc::new(ignored_paths)
    }
}
