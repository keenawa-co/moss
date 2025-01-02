use anyhow::Result;
use tauri::AppHandle;

use super::{
    instantiation::{InstantiationType, ServiceCollection, ServiceHandle},
    service::AnyService,
};

pub struct AppManager {
    services: ServiceCollection,
}

unsafe impl Send for AppManager {}
unsafe impl Sync for AppManager {}

impl AppManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            services: ServiceCollection::new(app_handle),
        }
    }

    pub fn with_service<T, F>(self, service: F, activation_type: InstantiationType) -> Self
    where
        T: AnyService + 'static,
        F: FnOnce(&AppHandle) -> T + 'static,
    {
        self.services.register(service, activation_type);
        self
    }

    pub fn service<T: AnyService>(&self) -> Result<ServiceHandle<T>> {
        self.services.get()
    }
}
