pub mod event;
pub mod event_registry;
pub mod hook;

use event_registry::EventRegistry;
use parking_lot::RwLock;

pub struct Context {
    event_registry: RwLock<EventRegistry>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            event_registry: RwLock::new(EventRegistry::new()),
        }
    }

    pub fn get_event_registry_mut<T>(&self, f: impl FnOnce(&mut EventRegistry) -> T) -> T {
        f(&mut self.event_registry.write())
    }

    pub fn get_event_registry<T>(&self, f: impl FnOnce(&EventRegistry) -> T) -> T {
        f(&self.event_registry.read())
    }
}
