use std::cell::{RefCell, UnsafeCell};

use derive_more::{Deref, DerefMut};
use moss_desktop::app::service::AnyService;
use tauri::plugin::Plugin as TauriPlugin;
use tauri::{EventLoopMessage, Runtime, Wry};

pub struct ServiceStore {
    store: Vec<Box<dyn AnyService>>,
}

impl Default for ServiceStore {
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

impl ServiceStore {
    /// Adds a service to the store.
    ///
    /// Returns `true` if a plugin with the same name is already in the store.
    pub fn register(&mut self, service: Box<dyn AnyService>) -> bool {
        let len = self.store.len();
        self.store.retain(|p| p.name() != service.name());
        let result = len != self.store.len();
        self.store.push(service);
        result
    }
}

type TauriBuilder = tauri::Builder<tauri_runtime_wry::Wry<EventLoopMessage>>;

// pub struct Builder {
//     tauri_builder: TauriBuilder,
//     services: ServiceStore,
// }

// impl Builder {
//     pub fn new() -> Self {
//         Self {
//             tauri_builder: tauri::Builder::default(),
//             services: ServiceStore::default(),
//         }
//     }

//     pub fn service<S: AnyService>(mut self, service: S) -> Self {
//         self.services.register(Box::new(service));
//         self
//     }

//     #[must_use]
//     pub fn plugin<P: TauriPlugin<Wry> + 'static>(mut self, plugin: P) -> Self {
//         self.tauri_builder = self.tauri_builder.plugin(plugin);
//         self
//     }
// }

// pub trait MyExtension {
//     fn service(self) -> Self;
// }

// impl MyExtension for TauriBuilder {
//     fn service(self) -> Self {
//        self.
//         println!("Hello, from MyExtension");

//         self
//     }
// }
