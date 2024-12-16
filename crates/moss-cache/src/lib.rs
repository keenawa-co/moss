pub mod moka;

use dashmap::DashMap;
use std::any::{type_name, Any};
use std::sync::Arc;
use std::time::{Duration, Instant};

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

    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError> {
        match self.content.clone().downcast::<T>() {
            Ok(result) => Ok(result),
            Err(_) => Err(CacheError::TypeMismatch {
                key: key.to_string(),
                type_name: type_name::<T>().to_string(),
            }),
        }
    }
}

trait CacheItem {
    fn new(item: DynType, ttl: Duration) -> Self;
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError>;
}

trait CacheBackend {
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T, ttl: Duration);
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError>;
    fn delete(&self, key: &str);
    fn contains(&self, key: &str) -> bool;
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

pub trait CacheFrontend<P: CacheBackend> {
    fn new(max_capacity: u64) -> Self;
    fn register<T: Any + Send + Sync>(
        &self,
        key: &str,
        ttl: Duration,
        updater: impl Fn() -> T + 'static,
    ) -> Arc<T>;
    fn deregister(&self, key: &str) -> Result<(), CacheError>;
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T, ttl: Duration);
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError>;
    fn delete(&self, key: &str);
    fn take<T: Any + Clone + Send + Sync>(&self, key: &str) -> Result<T, CacheError>;
    fn contains(&self, key: &str) -> bool;
}

pub struct MossCache<P: CacheBackend> {
    provider: P,
    retrievers: Retrievers,
}
