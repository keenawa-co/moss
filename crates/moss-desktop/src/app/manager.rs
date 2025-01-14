use anyhow::Result;
use moss_extension_point::registry::Registry;
use tauri::AppHandle;

use super::{
    instantiation::InstantiationType,
    service::{Service, ServiceCollection, ServiceHandle},
};

pub struct AppManager<'a> {
    services: ServiceCollection,
    registry: Registry<'a>,
}

unsafe impl<'a> Send for AppManager<'a> {}
unsafe impl<'a> Sync for AppManager<'a> {}

impl<'a> AppManager<'a> {
    pub fn new(app_handle: AppHandle, registry: Registry<'a>) -> Self {
        Self {
            services: ServiceCollection::new(app_handle),
            registry,
        }
    }

    pub fn with_service<T, F>(self, service: F, activation_type: InstantiationType) -> Self
    where
        T: Service + 'static,
        F: FnOnce(&AppHandle) -> T + 'static,
    {
        self.services.register(service, activation_type);
        self
    }

    pub fn service<T: Service>(&self) -> Result<ServiceHandle<T>> {
        self.services.get()
    }
}
