use anyhow::Result;
use hashbrown::{hash_map::Entry as HashMapEntry, HashMap};
use parking_lot::Mutex;
use std::{any::TypeId, future::Future, sync::Arc};
use tracing::warn;

use super::event::Event;
use super::hook::{AsyncHook, Hook};

pub struct EventRegistry {
    event_types: HashMap<&'static str, TypeId, ahash::RandomState>,
    hooks: HashMap<&'static str, Vec<Hook>, ahash::RandomState>,
    async_hooks: HashMap<&'static str, Vec<AsyncHook>, ahash::RandomState>,
}

impl EventRegistry {
    pub(crate) fn new() -> Self {
        Self {
            event_types: HashMap::with_hasher(ahash::RandomState::with_seed(42)),
            hooks: HashMap::with_hasher(ahash::RandomState::with_seed(42)),
            async_hooks: HashMap::with_hasher(ahash::RandomState::with_seed(42)),
        }
    }

    pub fn register_event<E>(&mut self)
    where
        E: Event + 'static,
    {
        let ty = TypeId::of::<E>();
        let type_id = E::TYPE_NAME;

        match self.event_types.entry(type_id) {
            HashMapEntry::Occupied(entry) => {
                if entry.get() == &ty {
                    panic!("Event with type {type_id} is already registered")
                } else {
                    panic!("Several events of type {type_id} were registered")
                }
            }
            HashMapEntry::Vacant(entry) => {
                entry.insert(ty);

                self.hooks.insert(type_id, Vec::new());
                self.async_hooks.insert(type_id, Vec::new());
            }
        }
    }

    pub fn register_hook<E>(&mut self, hook_fn: impl Fn(&mut E) -> Result<()> + Send + Sync)
    where
        E: Event + 'static,
    {
        let type_id = E::TYPE_NAME;
        let hook = Hook::new(hook_fn);

        self.hooks.get_mut(type_id).unwrap().push(hook);
    }

    pub fn register_async_hook<E, Fut>(
        &mut self,
        hook_fn: impl Fn(Arc<Mutex<E>>) -> Fut + 'static + Send + Sync,
    ) where
        E: Event + 'static,
        Fut: Future<Output = Result<()>> + 'static + Send,
    {
        let type_id = E::TYPE_NAME;
        let hook = AsyncHook::new(hook_fn);

        self.async_hooks.get_mut(type_id).unwrap().push(hook);
    }

    pub fn dispatch_event<'a, E>(&'a self, mut event: E)
    where
        E: Event + 'static,
    {
        let type_name = E::TYPE_NAME;

        let Some(_) = self.event_types.get(type_name) else {
            warn!("An event of an unknown {type_name} was dispatched");
            return;
        };

        let Some(hooks) = self.hooks.get(type_name) else {
            // A panic here guarantees the presence of a bug in the program
            panic!("The event {type_name} is registered but was not initialized",)
        };

        let Some(async_hooks) = self.async_hooks.get(type_name) else {
            // A panic here guarantees the presence of a bug in the program
            panic!("The event {type_name} is registered but was not initialized",)
        };

        // We don’t want to spawns if my event does not have asynchronous operations.
        if async_hooks.is_empty() {
            for hook in hooks {
                if let Err(e) = hook.call(&mut event) {
                    println!("{e}"); // TODO: handle error
                }
            }
        } else {
            // let event_clone = event.clone();
            // let hooks_clone = hooks.clone();
            // let async_hooks_clone = async_hooks.clone();

            // let sync_handle = tokio::spawn(async move {
            //     for hook in hooks_clone {
            //         if let Err(e) = hook.call(&mut event) {
            //             println!("{e}"); // TODO: handle error
            //         }
            //     }
            // });

            // let async_handle = tokio::spawn(async move {
            //     let mut tasks = Vec::with_capacity(async_hooks_clone.len());
            //     let guarded_event = Arc::new(Mutex::new(event_clone));

            //     for async_hook in async_hooks_clone {
            //         let guarded_event_clone = Arc::clone(&guarded_event);
            //         tasks.push(tokio::spawn(async move {
            //             if let Err(e) = async_hook.call(guarded_event_clone).await {
            //                 println!("{e}"); // TODO: handle error
            //             }
            //         }));
            //     }
            //     futures::future::join_all(tasks).await;
            // });

            // let _ = tokio::join!(sync_handle, async_handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MyEvent<'a> {
        user: &'a str,
    }

    unsafe impl<'a> super::Event for MyEvent<'a> {
        const TYPE_NAME: &'static str = "MyEvent";
    }

    #[test]
    fn event_pool_test() {
        let mut pool = EventRegistry::new();

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
