use crate::{Cache, CacheBackend, CacheError, DynType};
use moka::sync::{Cache as MokaCache, CacheBuilder as MokaCacheBuilder};
use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

pub struct MokaBackend {
    cache: MokaCache<String, DynType>,
}

impl MokaBackend {
    pub fn new(max_capacity: u64, ttl: Duration) -> Self {
        Self {
            cache: MokaCacheBuilder::new(max_capacity)
                .time_to_live(ttl)
                .build(),
        }
    }
}

impl CacheBackend for MokaBackend {
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T) {
        self.cache.insert(key.to_string(), DynType::new(val));
    }
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.cache.get(key) {
            Some(dyn_item) => dyn_item.get_concrete::<T>(key),
            None => Err(CacheError::NonexistentKey {
                key: key.to_string(),
            }),
        }
    }
    fn delete(&self, key: &str) {
        self.cache.remove(key);
    }

    fn contains(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
}

pub type MokaSyncCache = Cache<MokaBackend>;
