mod newcache;
use std::sync::LazyLock;
use std::{thread, time};

static cache: LazyLock<newcache::MyCache> = LazyLock::new(|| newcache::MyCache::new(0.8, 0.9));

fn main() {
    let cache_handle = LazyLock::force(&cache);
    let ths = (0..10)
        .map(|i| {
            thread::Builder::new()
                .name(format!("Thread {}", i))
                .spawn(move || {
                    fn get_random_number() -> i32 {
                        rand::random::<i32>() % 10
                    }
                    thread::sleep(time::Duration::from_secs(rand::random::<u64>() % 10));
                    cache_handle.register(&i.to_string(), i as i32, (i + 1) * 1000, || {
                        Box::new(get_random_number())
                    });
                    println!("Thread {}", i);
                    for round in (0..10) {
                        thread::sleep(time::Duration::from_secs(rand::random::<u64>() % 10));
                        println!(
                            "Thread {}, round {}, value: {}",
                            i,
                            round,
                            cache_handle.get::<i32>(&i.to_string()).unwrap()
                        );
                    }
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    ths.into_iter().for_each(|t| t.join().unwrap());
}

mod tests {}
