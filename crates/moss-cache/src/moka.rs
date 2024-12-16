use crate::{CacheBackend, CacheError, DynType, MossCache, Retrievers};
use dashmap::DashMap;
use moka::sync::{Cache as MokaCache, CacheBuilder as MokaCacheBuilder};
use moka::Expiry;
use std::any::Any;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct MokaCacheBackend {
    cache: MokaCache<String, DynType>,
}

impl CacheBackend for MokaCacheBackend {
    fn new(max_capacity: u64, ttl: Duration) -> Self {
        Self {
            cache: MokaCacheBuilder::new(max_capacity)
                .time_to_live(ttl)
                .build(),
        }
    }
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

impl<P> MossCache<P>
where
    P: CacheBackend,
{
    fn new(max_capacity: u64, ttl: Duration) -> Self {
        Self {
            provider: P::new(max_capacity, ttl),
            retrievers: Retrievers {
                funcs: DashMap::new(),
            },
        }
    }

    fn register<T: Any + Send + Sync>(
        &self,
        key: &str,
        updater: impl Fn() -> T + 'static,
    ) -> Arc<T> {
        let initial_val = updater();
        let retriever = {
            let updater = Arc::new(updater);
            Arc::new(move || DynType::new(updater()))
        };
        self.retrievers.register(key, retriever);
        self.provider.insert(key, initial_val);
        self.provider.get::<T>(key).unwrap()
    }

    fn deregister(&self, key: &str) -> Result<(), CacheError> {
        self.provider.delete(key);
        self.retrievers.deregister(key)
    }

    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T) {
        self.provider.insert(key, val);
    }

    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.provider.get::<T>(key) {
            Ok(val) => Ok(val),
            Err(_) => self.retrievers.get(key)?().get_concrete::<T>(key),
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
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32i32);
    }
    #[test]
    fn test_cache_delete() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        cache.delete("i32");
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_delete_nonexistent() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.delete("nonexistent");
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_cache_get() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }

    #[test]
    #[should_panic]
    fn test_cache_get_incorrect_type() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        cache.get::<String>("i32").unwrap();
    }

    #[test]
    fn test_cache_take() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        let i: i32 = cache.take::<i32>("i32").unwrap();
        assert_eq!(i, 32);
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_expiry() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(500));
        cache.insert::<i32>("i32", 32i32);
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_register() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(500));
        cache.register::<i32>("i32", || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn example() {
        let cache = MokaSyncCache::new(1024, Duration::from_millis(1000));
        cache.register::<String>("file", || std::fs::read_to_string("test.txt").unwrap());
        println!("{:?}", cache.get::<String>("file"));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("file"));
        assert!(cache.get::<String>("file").is_ok());
    }
}
