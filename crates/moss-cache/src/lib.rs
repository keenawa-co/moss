use dashmap::DashMap;
use moka::sync::{Cache as MokaCache, CacheBuilder as MokaCacheBuilder};
use moka::Expiry;
use std::any::{type_name, Any};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

type DynType = Arc<dyn Any + Send + Sync>;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("The value of key '{key}' does not have type '{type_name}'")]
    TypeMismatch { key: String, type_name: String },

    #[error("The key '{key}' does not exist")]
    NonexistentKey { key: String },
}

#[derive(Clone, Debug)]
struct CacheItem {
    pub item: DynType,
    ttl: Duration,
}

impl CacheItem {
    pub(crate) fn get<T>(&self, key: &str) -> Result<Arc<T>, CacheError>
    where
        T: Any + Send + Sync,
    {
        match self.item.clone().downcast::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(CacheError::TypeMismatch {
                key: key.to_string(),
                type_name: type_name::<T>().to_string(),
            }),
        }
    }
}

pub struct CustomExpiry {}
impl Expiry<String, CacheItem> for CustomExpiry {
    fn expire_after_create(
        &self,
        key: &String,
        value: &CacheItem,
        created_at: Instant,
    ) -> Option<Duration> {
        Some(value.ttl)
    }

    fn expire_after_read(
        &self,
        key: &String,
        value: &CacheItem,
        read_at: Instant,
        duration_until_expiry: Option<Duration>,
        last_modified_at: Instant,
    ) -> Option<Duration> {
        Some(value.ttl)
    }
}

impl CacheItem {
    fn new(item: DynType, ttl: Duration) -> CacheItem {
        CacheItem { item, ttl }
    }
}

type DynamicTypeCache = MokaCache<String, CacheItem>;

pub struct Cache {
    cache: MokaCache<String, CacheItem>,
    retrievers: DashMap<String, Arc<dyn Fn() -> CacheItem>>,
}

impl Cache {
    pub fn new(max_capacity: u64) -> Cache {
        Cache {
            cache: MokaCacheBuilder::new(max_capacity)
                .expire_after(CustomExpiry {})
                .build(),
            retrievers: DashMap::new(),
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
    pub fn insert<T>(&self, key: &str, value: T, ttl: Duration)
    where
        T: Any + Send + Sync,
    {
        self.cache
            .insert(key.to_string(), CacheItem::new(Arc::new(value), ttl));
    }

    pub fn delete(&self, key: &str) {
        self.cache.remove(key);
    }

    pub fn get<T>(&self, key: &str) -> Result<Arc<T>, CacheError>
    where
        T: Any + Send + Sync,
    {
        match self.cache.get(key) {
            Some(cache_item) => cache_item.get::<T>(key),
            None => {
                let retriever = self
                    .retrievers
                    .get(key)
                    .ok_or(CacheError::NonexistentKey {
                        key: key.to_string(),
                    })?
                    .value()
                    .clone();
                self.cache
                    .get_with(key.to_string(), move || retriever())
                    .get::<T>(key)
            }
        }
    }

    pub fn take<T>(&self, key: &str) -> Result<T, CacheError>
    where
        T: Any + Clone + Send + Sync,
    {
        let cache_item = self.cache.remove(key).ok_or(CacheError::NonexistentKey {
            key: key.to_string(),
        })?;
        cache_item
            .get::<T>(key)
            .map(|arc_t| Arc::<T>::unwrap_or_clone(arc_t))
    }

    pub fn register<T>(&self, key: &str, ttl: Duration, update: impl Fn() -> T + 'static) -> Arc<T>
    where
        T: Any + Send + Sync,
    {
        let update = Arc::new(update);
        let retriever = {
            let update = Arc::clone(&update);
            Arc::new(move || CacheItem::new(Arc::new(update()), ttl))
        };
        self.retrievers.insert(key.to_string(), retriever.clone());
        self.cache
            .get_with(key.to_string(), move || retriever())
            .get::<T>(key)
            .unwrap()
    }

    pub fn deregister(&self, key: &str) -> Result<(), CacheError> {
        self.cache.invalidate(key);
        match self.retrievers.remove(key) {
            Some(retriever) => Ok(()),
            None => Err(CacheError::NonexistentKey {
                key: key.to_string(),
            }),
        }
    }
}

mod test {
    use super::*;
    use std::thread;
    #[test]
    fn test_cache_insert() {
        let cache = Cache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn test_cache_delete() {
        let cache = Cache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        cache.delete("i32");
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_delete_nonexistent() {
        let cache = Cache::new(1024);
        cache.delete("nonexistent");
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_cache_get() {
        let cache = Cache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }

    #[test]
    #[should_panic]
    fn test_cache_get_incorrect_type() {
        let cache = Cache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        cache.get::<String>("i32").unwrap();
    }

    #[test]
    fn test_cache_take() {
        let cache = Cache::new(1024);
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        let i: i32 = cache.take::<i32>("i32").unwrap();
        assert_eq!(i, 32);
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_expiry() {
        let cache = Cache::new(1024);
        cache.insert::<i32>("i32", 32i32, Duration::from_millis(500));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_register() {
        let cache = Cache::new(1024);
        cache.register::<i32>("i32", Duration::from_millis(500), || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn example() {
        let cache = Cache::new(1024);
        cache.register::<String>("file", Duration::from_millis(100), || {
            std::fs::read_to_string("test.txt").unwrap()
        });
        println!("{:?}", cache.get::<String>("file"));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("file"));
        assert!(cache.get::<String>("file").is_ok());
    }
}
