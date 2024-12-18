use crate::CacheError;
use std::any::Any;
use std::sync::Arc;
use std::time::Duration;

pub mod moka;

pub(crate) trait CacheBackend {
    fn new(max_capacity: u64, ttl: Duration) -> Self;
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T);
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError>;
    fn delete(&self, key: &str);
    fn contains(&self, key: &str) -> bool;
}
