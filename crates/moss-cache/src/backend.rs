pub mod moka;

use crate::CacheError;
use std::any::Any;
use std::sync::Arc;

pub trait CacheBackend {
    fn insert<T: Any + Send + Sync>(&self, key: &str, val: T);
    fn get<T: Any + Send + Sync>(&self, key: &str) -> Result<Arc<T>, CacheError>;
    fn delete(&self, key: &str);
    fn contains(&self, key: &str) -> bool;
}
