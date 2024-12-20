pub mod backend;

use dashmap::DashMap;
use std::any::{type_name, Any};
use std::sync::Arc;
use thiserror::Error;

use crate::backend::CacheBackend;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("The value of key '{key}' has the type name of '{type_name}'")]
    TypeMismatch {
        key: String,
        type_name: &'static str,
    },

    #[error("The key '{key}' does not exist")]
    NonexistentKey { key: String },
}

#[derive(Clone, Debug)]
struct DynType {
    content: Arc<dyn Any + Send + Sync>,
    type_name: &'static str,
}

impl DynType {
    fn new<T>(content: T) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            content: Arc::new(content),
            type_name: type_name::<T>(),
        }
    }

    fn get_concrete<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.content.clone().downcast::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(CacheError::TypeMismatch {
                key: key.to_string(),
                type_name: self.type_name,
            }),
        }
    }
}

type RetrieverCallback = Arc<dyn Fn() -> DynType + Send + Sync>;

struct Retrievers {
    funcs: DashMap<String, RetrieverCallback>,
}

impl Retrievers {
    fn new() -> Retrievers {
        Self {
            funcs: DashMap::new(),
        }
    }

    fn register(&self, key: &str, updater: RetrieverCallback) {
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

    fn get(&self, key: &str) -> Result<RetrieverCallback, CacheError> {
        self.funcs
            .get(key)
            .map(|f| f.value().clone())
            .ok_or_else(|| CacheError::NonexistentKey {
                key: key.to_string(),
            })
    }
}

pub struct Cache<B: CacheBackend> {
    backend: B,
    retrievers: Retrievers,
}

impl<B: CacheBackend> Cache<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            retrievers: Retrievers::new(),
        }
    }
}

impl<B> Cache<B>
where
    B: CacheBackend,
{
    pub fn register<T: Any + Send + Sync>(
        &self,
        key: &str,
        updater: impl Fn() -> T + Send + Sync + 'static,
    ) -> Arc<T> {
        let initial_val = updater();
        let retriever = {
            let updater = Arc::new(updater);
            Arc::new(move || DynType::new(updater()))
        };
        self.retrievers.register(key, retriever);
        self.backend.insert(key, initial_val);
        self.backend
            .get::<T>(key)
            .expect("Just inserted value must be retrievable")
    }

    pub fn deregister(&self, key: &str) -> Result<(), CacheError> {
        self.backend.delete(key);
        self.retrievers.deregister(key)
    }

    pub fn insert<T>(&self, key: &str, val: T)
    where
        T: Any + Send + Sync + Clone,
    {
        self.backend.insert(key, val);
    }

    pub fn get<T>(&self, key: &str) -> Result<Arc<T>, CacheError>
    where
        T: Any + Send + Sync + Clone,
    {
        match self.backend.get::<T>(key) {
            Ok(val) => Ok(val),
            Err(CacheError::TypeMismatch { key, type_name }) => {
                Err(CacheError::TypeMismatch { key, type_name })
            }
            Err(CacheError::NonexistentKey { key: _ }) => {
                // The value is missing in the backend, try to retrieve it using the retriever
                let retriever = self.retrievers.get(key)?;
                let new_val_dyn = retriever();

                // Here we are confident that if the retriever returns a DynType, it is the correct type we expect.
                let new_val = new_val_dyn
                    .get_concrete::<T>(key)
                    .expect("The retriever must return the correct type");
                let extracted = match Arc::try_unwrap(new_val) {
                    Ok(val) => val,
                    Err(arc_val) => (*arc_val).clone(),
                };
                self.backend.insert(key, extracted);
                self.backend.get::<T>(key)
            }
        }
    }

    pub fn delete(&self, key: &str) {
        self.backend.delete(key);
    }

    pub fn take<T>(&self, key: &str) -> Result<T, CacheError>
    where
        T: Any + Send + Sync + Clone,
    {
        let cached_item = self.get::<T>(key)?;
        self.delete(key);
        match Arc::try_unwrap(cached_item) {
            Ok(val) => Ok(val),
            Err(arc_val) => Ok((*arc_val).clone()),
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.backend.contains(key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        thread,
        time::{Duration, Instant},
    };

    struct MockBackend {
        cache: DashMap<String, DynType>,
        entry_timestamp: DashMap<String, Instant>,
        ttl: Duration,
    }

    impl MockBackend {
        fn new(ttl: Duration) -> Self {
            Self {
                cache: DashMap::new(),
                entry_timestamp: DashMap::new(),
                ttl,
            }
        }
    }

    impl CacheBackend for MockBackend {
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
    fn test_cache_type_mismatch_error_message() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        let result = cache.get::<String>("i32");
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(
                err.to_string(),
                format!(
                    "The value of key '{}' has the type name of '{}'",
                    "i32", "i32"
                )
            );
        }
    }

    #[test]
    fn test_cache_insert_single_type() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32i32);
    }

    #[test]
    fn test_cache_insert_multiple_types() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
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
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        assert!(cache.contains("i32"));
        cache.delete("i32");
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_delete_nonexistent() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        assert!(!cache.contains("nonexistent"));
        cache.delete("nonexistent");
        assert!(!cache.contains("nonexistent"));
    }

    #[test]
    fn test_cache_get_with_correct_type_and_key() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }

    #[test]
    fn test_cache_get_with_incorrect_type() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        assert!(matches!(
            cache.get::<String>("i32"),
            Err(CacheError::TypeMismatch { .. })
        ))
    }

    #[test]
    fn test_cache_get_with_nonexistent_key() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        assert!(matches!(
            cache.get::<i32>("i32"),
            Err(CacheError::NonexistentKey { .. })
        ))
    }

    #[test]
    fn test_cache_take_existent_value() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        let i: i32 = cache.take::<i32>("i32").unwrap();
        assert_eq!(i, 32);
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_take_with_incorrect_type() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        cache.insert("i32", 32i32);
        assert!(matches!(
            cache.take::<String>("i32"),
            Err(CacheError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn test_cache_take_with_nonexistent_key() {
        let mock_backend = MockBackend::new(Duration::from_millis(1000));
        let cache = MockCache::new(mock_backend);
        assert!(matches!(
            cache.take::<i32>("i32"),
            Err(CacheError::NonexistentKey { .. })
        ));
    }

    #[test]
    fn test_cache_expire_beyond_ttl() {
        let mock_backend = MockBackend::new(Duration::from_millis(500));
        let cache = MockCache::new(mock_backend);
        cache.insert::<i32>("i32", 32i32);
        assert!(cache.contains("i32"));
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
    }

    #[test]
    fn test_cache_register_initialize_value() {
        let mock_backend = MockBackend::new(Duration::from_millis(500));
        let cache = MockCache::new(mock_backend);
        cache.register::<i32>("i32", || 32);
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
    }

    #[test]
    fn test_cache_register_update_expired_value() {
        let mock_backend = MockBackend::new(Duration::from_millis(500));
        let cache = MockCache::new(mock_backend);
        cache.register::<i32>("i32", || 32);
        thread::sleep(Duration::from_millis(1000));
        assert!(!cache.contains("i32"));
        assert_eq!(*cache.get::<i32>("i32").unwrap(), 32);
        assert!(cache.contains("i32"));
    }
}
