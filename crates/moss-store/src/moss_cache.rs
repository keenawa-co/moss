use dashmap::DashMap;
use moka::sync::{Cache, CacheBuilder};
use moka::Expiry;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

type DynType = Arc<dyn Any + Send + Sync>;

#[derive(Clone, Debug)]
struct MossCacheItem {
    pub item: DynType,
    ttl: Duration,
}

impl MossCacheItem {
    pub fn get<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        self.item.clone().downcast::<T>().ok().map(|i| i)
    }
}

pub struct CustomExpiry {}
impl Expiry<String, MossCacheItem> for CustomExpiry {
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

    fn expire_after_create(
        &self,
        key: &String,
        value: &MossCacheItem,
        created_at: Instant,
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
    fn new(max_capacity: u64) -> MossCacheBuilder {
        MossCacheBuilder {
            cache_builder: CacheBuilder::new(max_capacity).expire_after(CustomExpiry {}),
        }
    }

    fn build(self) -> MossCache {
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
    fn get<T>(&self, key: &str) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        /// Attempt to first get the value of the key; if not, it will use the registered function.
        match self.cache.get(key) {
            Some(cache_item) => cache_item.get::<T>(),
            None => {
                let retriever = self.retrievers.get(key)?.value().clone();
                self.cache
                    .get_with(key.to_string(), move || retriever())
                    .get::<T>()
            }
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    fn register<T>(&self, key: &str, ttl: Duration, update: impl Fn() -> T + 'static) -> Arc<T>
    where
        T: Any + Send + Sync,
    {
        /// Register a key with an update function, which will be called when the entry is expired and removed
        let update = Arc::new(update);
        let retriever = {
            let update = Arc::clone(&update);
            Arc::new(move || MossCacheItem::new(Arc::new(update()), ttl))
        };
        self.retrievers.insert(key.to_string(), retriever.clone());
        self.cache
            .get_with(key.to_string(), move || retriever())
            .get::<T>()
            .unwrap()
    }

    fn deregister(&self, key: &str) -> Option<()> {
        /// Delete cached value and the registered function
        /// If no function has been registered for this key, returns None
        /// Otherwise returns Some(())
        self.cache.invalidate(key);
        self.retrievers.remove(key).map(|_| ())
    }
}

mod test {
    use super::*;
    #[test]
    fn test_cache_simple() {
        let cache_builder = MossCacheBuilder::new(1024);
        let cache = cache_builder.build();
        cache.register::<i32>("i32", Duration::from_millis(500), || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn test_cache_expiry() {
        let cache_builder = MossCacheBuilder::new(1024);
        let cache = cache_builder.build();
        cache.register::<i32>("i32", Duration::from_millis(500), || 32);
        println!("{:?}", cache.cache);
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_update_expired_data() {
        let cache_builder = MossCacheBuilder::new(1024);
        let cache = cache_builder.build();
        cache.register::<i32>("i32", Duration::from_millis(500), || 32);
        thread::sleep(Duration::from_millis(1000));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }
}
