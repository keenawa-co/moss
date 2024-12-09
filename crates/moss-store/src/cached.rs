use cached::{Cached, TimedCache};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct MyCache {
    cache: Arc<Mutex<TimedCache<String, Box<dyn Any + Send>>>>,
    fn_register: Arc<Mutex<HashMap<String, fn() -> Box<dyn Any + Send>>>>,
    cool_down: Arc<Mutex<TimedCache<bool, bool>>>,
    hit_ratio_lower_limit: f64,
    hit_ratio_upper_limit: f64,
}

impl MyCache {
    pub fn new(
        lifespan: u64,
        refresh_on_retrieval: bool,
        hit_ratio_lower_limit: f64,
        hit_ratio_upper_limit: f64,
    ) -> MyCache {
        MyCache {
            cache: Arc::new(Mutex::new(TimedCache::with_lifespan_and_refresh(
                lifespan,
                refresh_on_retrieval,
            ))),
            fn_register: Arc::new(Mutex::new(HashMap::new())),
            cool_down: Arc::new(Mutex::new(TimedCache::with_lifespan(lifespan))),
            hit_ratio_lower_limit,
            hit_ratio_upper_limit,
        }
    }

    pub fn cache_hits(&self) -> u64 {
        self.cache.lock().unwrap().cache_hits().unwrap_or(0)
    }

    pub fn cache_misses(&self) -> u64 {
        self.cache.lock().unwrap().cache_misses().unwrap_or(0)
    }

    pub fn calculate_hit_ratio(&self) -> f64 {
        let hits = self.cache_hits();
        let misses = self.cache_misses();
        let total = hits + misses;
        if total == 0 {
            1.0
        } else {
            hits as f64 / total as f64
        }
    }

    pub fn get_lifespan(&self) -> u64 {
        self.cache.lock().unwrap().cache_lifespan().unwrap_or(0)
    }

    pub fn set_lifespan(&mut self, lifespan: u64) {
        self.cache.lock().unwrap().cache_set_lifespan(lifespan);
    }

    pub fn adjust_lifespan(&mut self) {
        if self.cool_down.lock().unwrap().cache_size() != 0 {
            return;
        }
        let hit_ratio = self.calculate_hit_ratio();
        println!(
            "Current Hit Ratio {}, Desired Hit Ratio Range {} - {}",
            hit_ratio, self.hit_ratio_lower_limit, self.hit_ratio_upper_limit
        );
        if hit_ratio < self.hit_ratio_lower_limit {
            let diff = self.hit_ratio_lower_limit - hit_ratio;
            println!("Extending cache lifespan by {:.2}%", diff * 100.0);
            let current_lifespan = self.get_lifespan();
            let new_lifespan = (current_lifespan as f64 * (1.0 + diff).ceil()) as u64;
            println!("Current Lifespan {}", current_lifespan);
            println!("New Lifespan {}", new_lifespan);
            self.set_lifespan(new_lifespan);
        } else if hit_ratio > self.hit_ratio_upper_limit {
            let diff = hit_ratio - self.hit_ratio_upper_limit;
            println!("Shrinking cache lifespan by {:.2}%", diff * 100.0);
            let current_lifespan = self.get_lifespan();
            let new_lifespan = (current_lifespan as f64 * (1.0 + diff).ceil()) as u64;
            println!("Current Lifespan {}", current_lifespan);
            println!("New Lifespan {}", new_lifespan);
            self.set_lifespan(new_lifespan);
        }
        if let Ok(mut cool_down) = self.cool_down.lock() {
            cool_down.cache_set_lifespan(self.get_lifespan());
            cool_down.cache_set(true, true);
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        T: Any + Send,
    {
        self.cache
            .lock()
            .unwrap()
            .cache_set(key.to_string(), Box::new(value));
    }

    pub fn register<T>(&mut self, key: &str, value: T, fun: fn() -> Box<dyn Any + Send>)
    where
        T: Any + Send,
    {
        self.set(key, value);
        self.fn_register
            .lock()
            .unwrap()
            .insert(key.to_string(), fun);
    }

    pub fn deregister(&mut self, key: &str) {
        self.cache.lock().unwrap().cache_remove(key);
        self.fn_register.lock().unwrap().remove(key);
    }
    pub fn get<T: 'static + Send + Clone + Debug>(&mut self, key: &str) -> Option<T> {
        if let Some(cached) = self.cache.lock().unwrap().cache_get(key) {
            return (*(*cached)).downcast_ref::<T>().map(|value| value.clone());
        }

        println!("Cache Miss! {}", self.cache_misses());
        self.adjust_lifespan();

        let new_val = self.fn_register.lock().unwrap().get(key).unwrap()()
            .downcast::<T>()
            .unwrap();
        self.set(key, new_val.clone());
        Some(*new_val.clone())
    }
}

mod tests {
    use crate::cached::MyCache;
    use std::{thread, time};

    fn get_name() -> String {
        thread::current().name().unwrap().to_owned()
    }
    #[test]
    fn test1() {
        let mut cache = MyCache::new(1, true, 0.8, 0.9);
        cache.register("string.name", "Hongyu".to_string(), || Box::new(get_name()));
        println!("{}", cache.get::<String>("string.name").unwrap());
        thread::sleep(time::Duration::from_secs(2));
        println!("{}", cache.get::<String>("string.name").unwrap());
    }
}
