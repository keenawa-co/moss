mod ignore_list_repo_impl;

pub mod migration;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::interface::cache::{CacheAdapter, IgnoredListRepository};
use crate::interface::cache_adapter::ignore_list_repo_impl::IgnoreListRepositoryImpl;

#[derive(Debug)]
pub struct CacheSQLiteAdapter {
    ignored_list_repo: Arc<dyn IgnoredListRepository>,
}

impl CacheSQLiteAdapter {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            ignored_list_repo: Arc::new(IgnoreListRepositoryImpl::new(conn.clone())),
        }
    }
}

#[async_trait]
impl CacheAdapter for CacheSQLiteAdapter {
    fn ignored_list_repo(&self) -> Arc<dyn IgnoredListRepository> {
        Arc::clone(&self.ignored_list_repo)
    }
}
