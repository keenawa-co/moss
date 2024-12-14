use cached::{Cached, CanExpire, ExpiringValueCache};
use parking_lot::Mutex;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, Instant};

// Cache: Serialized objects
// get_raw() -> Value, get<T>()

// Slotmap: prevent double calculating hash
// quick_cache

const CACHE_CAPACITY: usize = 1024;
#[derive(Debug)]
struct CacheItem {
    pub data: Box<dyn Any + Send + Sync>, // Arc, WeakRef, internal/platform/core/src/context_v2/node.rs
    pub expiration_time: Instant,
}

#[derive(Clone, Debug)]
struct CacheMetadata {
    pub ttl_milliseconds: u64,
    pub hits: u64,
    pub misses: u64,
    pub fun: fn() -> Box<dyn Any + Send + Sync>,
}

impl CanExpire for CacheItem {
    fn is_expired(&self) -> bool {
        Instant::now() >= self.expiration_time
    }
}

#[derive(Debug)]
struct State {
    cache: ExpiringValueCache<String, CacheItem>,
    cache_metadata: HashMap<String, CacheMetadata>,
}

#[derive(Debug)]
pub struct MyCache {
    state: Mutex<State>,
    hit_ratio_lower_limit: f64,
    hit_ratio_upper_limit: f64,
}

impl MyCache {
    pub fn new(hit_ratio_lower_limit: f64, hit_ratio_upper_limit: f64) -> MyCache {
        MyCache {
            state: Mutex::new(State {
                cache: ExpiringValueCache::with_size(CACHE_CAPACITY),
                cache_metadata: HashMap::new(),
            }),
            hit_ratio_lower_limit,
            hit_ratio_upper_limit,
        }
    }

    pub fn register<T>(
        &self,
        key: &str,
        value: T,
        ttl_milliseconds: u64,
        fun: fn() -> Box<dyn Any + Send + Sync>,
    ) where
        T: Any + Send + Sync,
    {
        if let Ok(mut state) = self.state.lock() {
            state.cache.cache_set(
                key.to_string(),
                CacheItem {
                    data: Box::new(value),
                    expiration_time: Instant::now() + Duration::from_millis(ttl_milliseconds),
                },
            );
            state.cache_metadata.insert(
                key.to_string(),
                CacheMetadata {
                    ttl_milliseconds,
                    hits: 0,
                    misses: 0,
                    fun,
                },
            );
        }
    }

    pub fn deregister(&self, key: &str) {
        if let Ok(mut state) = self.state.lock() {
            state.cache.cache_remove(key);
            state.cache_metadata.remove(key);
        }
    }
    pub fn get<T: 'static + Send + Sync + Clone + Debug>(&self, key: &str) -> Option<T> {
        if let Ok(mut state) = self.state.lock() {
            // TODO: This is ugly
            // We can't have two mutable references on both cache_metadata and cache
            let mut new_metadata = state.cache_metadata.get(key)?.to_owned();
            let cache_item = state.cache.cache_get_mut(key);
            let mut value: Option<T> = None;

            match cache_item {
                Some(item) => {
                    println!("Cache Hit of {}", key);
                    new_metadata.hits += 1;
                    // Cache Hit
                    let total = new_metadata.hits + new_metadata.misses;
                    let hit_ratio = new_metadata.hits as f64 / total as f64;
                    if hit_ratio > self.hit_ratio_upper_limit {
                        // Heuristic to shrink ttl
                        let diff = hit_ratio - self.hit_ratio_upper_limit;
                        println!("Shrinking cache TTL of {} by {:.2}%", key, diff * 100.0);
                        let prev_ttl = new_metadata.ttl_milliseconds;
                        let new_ttl = (prev_ttl as f64 * (1.0 - diff)).floor() as u64;
                        println!("Previous TTL: {} Milliseconds", prev_ttl);
                        println!("New TTL: {} Milliseconds", new_ttl);
                        new_metadata.ttl_milliseconds = new_ttl;
                    }
                    value = (*(item.data))
                        .downcast_ref::<T>()
                        .map(|value| value.clone());

                    println!("Value: {:?}", value);
                    item.expiration_time =
                        Instant::now() + Duration::from_millis(new_metadata.ttl_milliseconds);
                }
                None => {
                    // Cache Miss
                    println!("Cache Miss of {}", key);
                    new_metadata.misses += 1;
                    let total = new_metadata.hits + new_metadata.misses;
                    let hit_ratio = new_metadata.hits as f64 / total as f64;
                    if hit_ratio < self.hit_ratio_lower_limit {
                        let diff = self.hit_ratio_lower_limit - hit_ratio;
                        println!("Extending cache TTL of {} by {:.2}%", key, diff * 100.0);
                        let prev_ttl = new_metadata.ttl_milliseconds;
                        let new_ttl = ((prev_ttl as f64 * (1.0 + diff)).ceil()) as u64;
                        println!("Previous TTL: {} Seconds", prev_ttl);
                        println!("New TTL: {} Seconds", new_ttl);
                        new_metadata.ttl_milliseconds = new_ttl;
                    }
                    let new_boxed_val = (new_metadata.fun)();
                    value = Some((*new_boxed_val).downcast_ref::<T>().unwrap().to_owned());
                    state.cache.cache_set(
                        key.to_string(),
                        CacheItem {
                            data: new_boxed_val,
                            expiration_time: Instant::now()
                                + Duration::from_millis(new_metadata.ttl_milliseconds),
                        },
                    );
                }
            }
            state.cache_metadata.insert(key.to_string(), new_metadata);
            return value;
        }
        None
    }
}
