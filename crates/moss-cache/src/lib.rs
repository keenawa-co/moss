use dashmap::DashMap;
use moka::sync::{Cache, CacheBuilder};
use moka::Expiry;
use std::any::{type_name, Any};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

type DynType = Arc<dyn Any + Send + Sync>;

#[derive(Debug, Error)]
pub enum MossCacheError {
    #[error("The value of key '{key}' does not have type '{type_name}'")]
    TypeMismatch { key: String, type_name: String },

    #[error("The key '{key}' does not exist")]
    NonexistentKey { key: String },
}

#[derive(Clone, Debug)]
struct MossCacheItem {
    pub item: DynType,
    ttl: Duration,
}

impl MossCacheItem {
    pub(crate) fn get<T>(&self, key: &str) -> Result<Arc<T>, MossCacheError>
    where
        T: Any + Send + Sync,
    {
        match self.item.clone().downcast::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(MossCacheError::TypeMismatch {
                key: key.to_string(),
                type_name: type_name::<T>().to_string(),
            }),
        }
    }
}

pub struct CustomExpiry {}
impl Expiry<String, MossCacheItem> for CustomExpiry {
    fn expire_after_create(
        &self,
        key: &String,
        value: &MossCacheItem,
        created_at: Instant,
    ) -> Option<Duration> {
        Some(value.ttl)
    }

    fn expire_after_read(
        &self,
        key: &String,
        value: &MossCacheItem,
        read_at: Instant,
        duration_until_expiry: Option<Duration>,
        last_modified_at: Instant,
    ) -> Option<Duration> {
        Some(value.ttl)
    }
}

impl MossCacheItem {
    fn new(item: DynType, ttl: Duration) -> MossCacheItem {
        MossCacheItem { item, ttl }
    }
}

type DynamicTypeCache = Cache<String, MossCacheItem>;

pub struct MossCacheBuilder {
    cache_builder: CacheBuilder<String, MossCacheItem, DynamicTypeCache>,
}

impl MossCacheBuilder {
    pub fn new(max_capacity: u64) -> MossCacheBuilder {
        MossCacheBuilder {
            cache_builder: CacheBuilder::new(max_capacity).expire_after(CustomExpiry {}),
        }
    }

    pub fn build(self) -> MossCache {
        MossCache {
            cache: self.cache_builder.build(),
            retrievers: DashMap::new(),
        }
    }
}

pub struct MossCache {
    cache: Cache<String, MossCacheItem>,
    retrievers: DashMap<String, Arc<dyn Fn() -> MossCacheItem>>,
}

impl MossCache {
    pub fn contains(&self, key: &str) -> bool {
        /// Check if there's stored cache value for key
        self.cache.contains_key(key)
    }
    pub fn insert<T>(&self, key: &str, value: T, ttl: Duration)
    where
        T: Any + Send + Sync,
    {
        /// Insert a key-value pair into the cache with a given TTL
        self.cache
            .insert(key.to_string(), MossCacheItem::new(Arc::new(value), ttl));
    }

    pub fn delete(&self, key: &str) {
        /// Deletes the stored value for key
        self.cache.remove(key);
    }

    pub fn get<T>(&self, key: &str) -> Result<Arc<T>, MossCacheError>
    where
        T: Any + Send + Sync,
    {
        /// Attempt to first get the value of the key; if not, it will first update the cache value
        /// using the registered update function
        match self.cache.get(key) {
            Some(cache_item) => cache_item.get::<T>(key),
            None => {
                let retriever = self
                    .retrievers
                    .get(key)
                    .ok_or(MossCacheError::NonexistentKey {
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

    pub fn take<T>(&self, key: &str) -> Result<T, MossCacheError>
    where
        T: Any + Clone + Send + Sync,
    {
        /// Removes the stored value for key and get the raw T value
        /// To get T from Arc<T>, T must be Clone
        let cache_item = self
            .cache
            .remove(key)
            .ok_or(MossCacheError::NonexistentKey {
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
        /// Register a key with an update function, which will be called when the entry is expired
        let update = Arc::new(update);
        let retriever = {
            let update = Arc::clone(&update);
            Arc::new(move || MossCacheItem::new(Arc::new(update()), ttl))
        };
        self.retrievers.insert(key.to_string(), retriever.clone());
        self.cache
            .get_with(key.to_string(), move || retriever())
            .get::<T>(key)
            .unwrap()
    }

    pub fn deregister(&self, key: &str) -> Option<()> {
        /// Delete cached value and the registered function
        /// If no function has been registered for this key, returns None
        /// Otherwise returns Some(())
        self.cache.invalidate(key);
        self.retrievers.remove(key).map(|_| ())
    }
}

mod test {
    use super::*;
    use std::thread;
    #[test]
    fn test_cache_insert() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn test_cache_delete() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        cache.delete("i32");
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_delete_nonexistent() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.delete("nonexistent");
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_cache_get() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }

    #[test]
    #[should_panic]
    fn test_cache_get_incorrect_type() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        cache.get::<String>("i32").unwrap();
    }

    #[test]
    fn test_cache_take() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.insert("i32", 32i32, Duration::from_millis(1000));
        let i: i32 = cache.take::<i32>("i32").unwrap();
        assert_eq!(i, 32);
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_expiry() {
        let cache = MossCacheBuilder::new(1024).build();
        cache.insert::<i32>("i32", 32i32, Duration::from_millis(500));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_register() {
        let cache_builder = MossCacheBuilder::new(1024);
        let cache = cache_builder.build();
        cache.register::<i32>("i32", Duration::from_millis(500), || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }
}
