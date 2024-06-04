use anyhow::Result;
use hashbrown::{hash_map::Entry as HashMapEntry, HashMap};

use parking_lot::RwLock;
use serde::Serialize;
use std::{
    any::TypeId,
    ptr::{self, NonNull},
};

struct Pool {
    // rx: smol::channel::Receiver<Event>,
}

struct Abstract(());

pub unsafe trait Event: Sized {
    const TYPE_NAME: &'static str;
}

pub struct Hook {
    data: NonNull<Abstract>,
    call: unsafe fn(NonNull<Abstract>, NonNull<Abstract>, NonNull<Abstract>),
}

unsafe impl Sync for Hook {}
unsafe impl Send for Hook {}

impl Hook {
    pub fn new<E: Event, F: Fn(&mut E) -> Result<()>>(hook: F) -> Self {
        unsafe fn call<E: Event, F: Fn(&mut E) -> Result<()>>(
            hook: NonNull<Abstract>,
            event: NonNull<Abstract>,
            result: NonNull<Abstract>,
        ) {
            let hook: NonNull<F> = hook.cast();
            let mut event: NonNull<E> = event.cast();
            let result: NonNull<Result<()>> = result.cast();
            let hook: &F = hook.as_ref();

            ptr::write(result.as_ptr(), hook(event.as_mut()))
        }

        unsafe {
            Hook {
                data: NonNull::new_unchecked(Box::into_raw(Box::new(hook)) as *mut Abstract),
                call: call::<E, F>,
            }
        }
    }

    pub fn call<E: Event>(&self, event: &mut E) -> Result<()> {
        let mut result = Ok(());

        unsafe {
            (self.call)(
                self.data,
                NonNull::from(event).cast(),
                NonNull::from(&mut result).cast(),
            );

            result
        }
    }
}

pub struct EventPool {
    tx: smol::channel::Sender<String>,
    rx: smol::channel::Receiver<String>,
    hook_map: RwLock<HashMap<&'static str, Vec<Hook>, ahash::RandomState>>,
}

impl EventPool {
    fn new() -> Self {
        let (tx, rx) = smol::channel::unbounded();
        let hook_map = HashMap::with_hasher(ahash::RandomState::with_seed(42));

        Self {
            tx,
            rx,
            hook_map: RwLock::new(hook_map),
        }
    }

    pub fn register_event<E: Event>(&self) {
        let type_id = E::TYPE_NAME;

        let mut map = self.hook_map.write();

        match map.entry(type_id) {
            HashMapEntry::Occupied(_) => {
                panic!("Event {type_id} is already registered")
            }
            HashMapEntry::Vacant(_) => {
                map.insert(type_id, Vec::new());
                dbg!(1);
            }
        }
    }

    pub fn register_hook<E: Event>(&self, hook_fn: impl Fn(&mut E) -> Result<()> + Send + Sync) {
        let type_id = E::TYPE_NAME;
        let hook = Hook::new(hook_fn);

        self.hook_map.write().get_mut(type_id).unwrap().push(hook);
    }

    pub fn dispatch_event<E: Event>(&self, mut event: E) {
        dbg!(2);
        if let Some(hooks) = self.hook_map.read().get(E::TYPE_NAME) {
            dbg!(3);
            for hook in hooks {
                if let Err(e) = hook.call(&mut event) {
                    println!("{e}");
                }
            }
        };
    }
}

pub struct Context {
    pub event_pool: EventPool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            event_pool: EventPool::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyEvent<'a> {
        user: &'a str,
    }

    unsafe impl<'a> super::Event for MyEvent<'a> {
        const TYPE_NAME: &'static str = "MyEvent";
    }

    #[test]
    fn hook_test() {
        let hook_fn = |e: &mut MyEvent| -> Result<()> {
            println!("{}", e.user);

            Ok(())
        };

        let mut test_event = MyEvent { user: "g10z3r" };

        let hook = Hook::new(hook_fn);
        hook.call(&mut test_event).unwrap();
    }

    #[test]
    fn event_pool_test() {
        let mut pool = EventPool::new();

        let hook_fn = |e: &mut MyEvent| -> Result<()> {
            println!("{}", e.user);

            Ok(())
        };

        pool.register_event::<MyEvent>();

        pool.register_hook(hook_fn);

        let test_event = MyEvent { user: "g10z3r" };

        pool.dispatch_event(test_event);
    }
}
