use crate::{Cache, CacheError, DynType};
use dashmap::DashMap;
use std::any::Any;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub mod moka;

pub(crate) trait CacheBackend {
    fn new(max_capacity: u64, ttl: Duration) -> Self;
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T);
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError>;
    fn delete(&self, key: &str);
    fn contains(&self, key: &str) -> bool;
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
