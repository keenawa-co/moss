use crate::{CacheBackend, CacheError, CacheFrontend, CacheItem, DynType, MossCache, Retrievers};
use dashmap::DashMap;
use moka::sync::{Cache as MokaCache, CacheBuilder as MokaCacheBuilder};
use moka::Expiry;
use std::any::Any;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct MokaCacheItem {
    pub item: DynType,
    ttl: Duration,
}

impl CacheItem for MokaCacheItem {
    fn new(item: DynType, ttl: Duration) -> MokaCacheItem {
        MokaCacheItem { item, ttl }
    }

    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        self.item.get(key)
    }
}

pub struct MokaExpiry {}
impl Expiry<String, MokaCacheItem> for MokaExpiry {
    fn expire_after_create(
        &self,
        key: &String,
        value: &MokaCacheItem,
        created_at: Instant,
    ) -> Option<Duration> {
        Some(value.ttl)
    }

    fn expire_after_read(
        &self,
        key: &String,
        value: &MokaCacheItem,
        read_at: Instant,
        duration_until_expiry: Option<Duration>,
        last_modified_at: Instant,
    ) -> Option<Duration> {
        Some(value.ttl)
    }
}

struct MokaCacheBackend {
    cache: MokaCache<String, MokaCacheItem>,
}

impl CacheBackend for MokaCacheBackend {
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T, ttl: Duration) {
        self.cache
            .insert(key.to_string(), MokaCacheItem::new(DynType::new(val), ttl));
    }
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.cache.get(key) {
            Some(moka_item) => moka_item.item.get::<T>(key),
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

impl CacheFrontend<MokaCacheBackend> for MossCache<MokaCacheBackend> {
    fn new(max_capacity: u64) -> Self {
        Self {
            provider: MokaCacheBackend {
                cache: MokaCacheBuilder::new(max_capacity)
                    .expire_after(MokaExpiry {})
                    .build(),
            },
            retrievers: Retrievers {
                funcs: DashMap::new(),
            },
        }
    }

    fn register<T: Any + Send + Sync>(
        &self,
        key: &str,
        ttl: Duration,
        updater: impl Fn() -> T + 'static,
    ) -> Arc<T> {
        let initial_val = updater();
        let retriever = {
            let updater = Arc::new(updater);
            Arc::new(move || DynType::new(updater()))
        };
        self.retrievers.register(key, retriever);
        self.insert(key, initial_val, ttl);
        self.get(key).unwrap()
    }

    fn deregister(&self, key: &str) -> Result<(), CacheError> {
        self.delete(key);
        self.retrievers.deregister(key)
    }

    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T, ttl: Duration) {
        self.provider.insert(key, val, ttl);
    }

    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.provider.get::<T>(key) {
            Ok(val) => Ok(val),
            Err(_) => self.retrievers.get(key)?().get::<T>(key),
        }
    }

    fn delete(&self, key: &str) {
        self.provider.delete(key);
    }

    fn take<T: Any + Clone + Send + Sync>(&self, key: &str) -> Result<T, CacheError> {
        let cached_item = self.get::<T>(key)?;
        self.delete(key);
        let cached_content = Arc::<T>::unwrap_or_clone(cached_item);
        Ok(cached_content)
    }

    fn contains(&self, key: &str) -> bool {
        self.provider.contains(key)
    }
}

pub type MokaSyncCache = MossCache<MokaCacheBackend>;

mod test {
    use super::*;
    use std::thread;
    #[test]
    fn test_cache_insert() {
        let cache = MokaSyncCache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32i32);
    }
    #[test]
    fn test_cache_delete() {
        let cache = MokaSyncCache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        cache.delete("i32");
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_delete_nonexistent() {
        let cache = MokaSyncCache::new(1024);
        cache.delete("nonexistent");
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_cache_get() {
        let cache = MokaSyncCache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }

    #[test]
    #[should_panic]
    fn test_cache_get_incorrect_type() {
        let cache = MokaSyncCache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        cache.get::<String>("i32").unwrap();
    }

    #[test]
    fn test_cache_take() {
        let cache = MokaSyncCache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        let i: i32 = cache.take::<i32>("i32").unwrap();
        assert_eq!(i, 32);
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_expiry() {
        let cache = MokaSyncCache::new(1024);
        cache.insert::<i32>("i32", 32i32, Duration::from_millis(500));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_register() {
        let cache = MokaSyncCache::new(1024);
        cache.register::<i32>("i32", Duration::from_millis(500), || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn example() {
        let cache = MokaSyncCache::new(1024);
        cache.register::<String>("file", Duration::from_millis(100), || {
            std::fs::read_to_string("test.txt").unwrap()
        });
        println!("{:?}", cache.get::<String>("file"));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("file"));
        assert!(cache.get::<String>("file").is_ok());
    }
}
