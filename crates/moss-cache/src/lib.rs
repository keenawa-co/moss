pub mod backend;

use dashmap::DashMap;
use std::any::{type_name, Any};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::backend::CacheBackend;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("The value of key '{key}' does not have type '{type_name}'")]
    TypeMismatch { key: String, type_name: String },

    #[error("The key '{key}' does not exist")]
    NonexistentKey { key: String },
}

#[derive(Clone, Debug)]
struct DynType {
    content: Arc<dyn Any + Send + Sync>,
}

impl DynType {
    fn new(content: impl Any + Send + Sync) -> Self {
        Self {
            content: Arc::new(content),
        }
    }

    fn get_concrete<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.content.clone().downcast::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(CacheError::TypeMismatch {
                key: key.to_string(),
                type_name: type_name::<T>().to_string(),
            }),
        }
    }
}

struct Retrievers {
    funcs: DashMap<String, Arc<dyn Fn() -> DynType>>,
}

impl Retrievers {
    fn new() -> Retrievers {
        Self {
            funcs: DashMap::new(),
        }
    }

    fn register(&self, key: &str, updater: Arc<dyn Fn() -> DynType>) {
        self.funcs.insert(key.to_string(), updater);
    }

    fn deregister(&self, key: &str) -> Result<(), CacheError> {
        match self.funcs.remove(key) {
            Some(_) => Ok(()),
            None => Err(CacheError::NonexistentKey {
                key: key.to_string(),
            }),
        }
    }

    fn get(&self, key: &str) -> Result<Arc<dyn Fn() -> DynType>, CacheError> {
        match self.funcs.get(key) {
            Some(func) => Ok(func.value().clone()),
            None => Err(CacheError::NonexistentKey {
                key: key.to_string(),
            }),
        }
    }
}

pub struct Cache<P: CacheBackend> {
    provider: P,
    retrievers: Retrievers,
}

impl<P> Cache<P>
where
    P: CacheBackend,
{
    pub fn new(max_capacity: u64, ttl: Duration) -> Self {
        Self {
            provider: P::new(max_capacity, ttl),
            retrievers: Retrievers {
                funcs: DashMap::new(),
            },
        }
    }

    pub fn register<T: Any + Send + Sync>(
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

    pub fn deregister(&self, key: &str) -> Result<(), CacheError> {
        self.provider.delete(key);
        self.retrievers.deregister(key)
    }

    pub fn insert<T: Any + Send + Sync>(&self, key: &str, val: T) {
        self.provider.insert(key, val);
    }

    pub fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.provider.get::<T>(key) {
            Ok(val) => Ok(val),
            Err(CacheError::TypeMismatch { key, type_name }) => {
                Err(CacheError::TypeMismatch { key, type_name })
            }
            Err(CacheError::NonexistentKey { .. }) => {
                let new_arc_value = self.retrievers.get(key)?().content.downcast::<T>().unwrap();
                let new_value = Arc::into_inner(new_arc_value).unwrap();
                self.provider.insert(key, new_value);
                self.provider.get(key)
            }
        }
    }

    pub fn delete(&self, key: &str) {
        self.provider.delete(key);
    }

    pub fn take<T: Any + Clone + Send + Sync>(&self, key: &str) -> Result<T, CacheError> {
        let cached_item = self.get::<T>(key)?;
        self.delete(key);
        let cached_content = Arc::<T>::unwrap_or_clone(cached_item);
        Ok(cached_content)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.provider.contains(key)
    }
}

mod test {
    use super::*;
    use std::thread;

    struct MockBackend {
        cache: DashMap<String, DynType>,
        entry_timestamp: DashMap<String, Instant>,
        ttl: Duration,
    }
    impl CacheBackend for MockBackend {
        fn new(max_capacity: u64, ttl: Duration) -> Self {
            Self {
                cache: DashMap::new(),
                entry_timestamp: DashMap::new(),
                ttl,
            }
        }

        fn insert<T: Any + Send + Sync>(&self, key: &str, val: T) {
            self.cache.insert(key.to_string(), DynType::new(val));
            self.entry_timestamp.insert(key.to_string(), Instant::now());
        }

        fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
            match self.entry_timestamp.get(key) {
                Some(entry_time) => {
                    if Instant::now() - *entry_time.value() > self.ttl {
                        Err(CacheError::NonexistentKey {
                            key: key.to_string(),
                        })
                    } else {
                        self.cache.get(key).unwrap().get_concrete::<T>(key)
                    }
                }
                None => Err(CacheError::NonexistentKey {
                    key: key.to_string(),
                }),
            }
        }

        fn delete(&self, key: &str) {
            self.cache.remove(key);
            self.entry_timestamp.remove(key);
        }

        fn contains(&self, key: &str) -> bool {
            match self.entry_timestamp.get(key) {
                Some(entry_time) => Instant::now() - *entry_time.value() < self.ttl,
                None => false,
            }
        }
    }

    type MockCache = Cache<MockBackend>;

    #[test]
    fn test_cache_insert_single_type() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32i32);
    }

    #[test]
    fn test_cache_insert_multiple_types() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("String", String::from("Hello, world!"));
        assert_eq!(
            *cache.get::<String>("String").unwrap(),
            String::from("Hello, world!")
        );

        cache.insert("bool", true);
        assert_eq!(*cache.get::<bool>("bool").unwrap(), true);

        cache.insert("Vec<i32>", vec![1, 2, 3]);
        assert_eq!(*cache.get::<Vec<i32>>("Vec<i32>").unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_cache_delete_existing_keys() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert!(cache.contains("i32"));
        cache.delete("i32");
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_delete_nonexistent() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        assert!(!cache.contains("nonexistent"));
        cache.delete("nonexistent");
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_cache_get_with_correct_type_and_key() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }

    #[test]
    fn test_cache_get_with_incorrect_type() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert!(matches!(
            cache.get::<String>("i32"),
            Err(CacheError::TypeMismatch { .. })
        ))
    }

    #[test]
    fn test_cache_get_with_nonexistent_key() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        assert!(matches!(
            cache.get::<i32>("i32"),
            Err(CacheError::NonexistentKey { .. })
        ))
    }

    #[test]
    fn test_cache_take_existent_value() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        let i: i32 = cache.take::<i32>("i32").unwrap();
        assert_eq!(i, 32);
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_take_with_incorrect_type() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        cache.insert("i32", 32i32);
        assert!(matches!(
            cache.take::<String>("i32"),
            Err(CacheError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn test_cache_take_with_nonexistent_key() {
        let cache = MockCache::new(1024, Duration::from_millis(1000));
        assert!(matches!(
            cache.take::<i32>("i32"),
            Err(CacheError::NonexistentKey { .. })
        ));
    }

    #[test]
    fn test_cache_expire_beyond_ttl() {
        let cache = MockCache::new(1024, Duration::from_millis(500));
        cache.insert::<i32>("i32", 32i32);
        assert!(cache.contains("i32"));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_register_initialize_value() {
        let cache = MockCache::new(1024, Duration::from_millis(500));
        cache.register::<i32>("i32", || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn test_cache_register_update_expired_value() {
        let cache = MockCache::new(1024, Duration::from_millis(500));
        cache.register::<i32>("i32", || 32);
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }
}
