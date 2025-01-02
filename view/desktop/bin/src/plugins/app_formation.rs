use std::sync::Arc;

use moss_desktop::app::{
    lifecycle::LifecycleManager,
    service::{
        ActivationPoint, AnyService, ServiceManager, ServiceMetadata, MAX_ACTIVATION_POINTS,
    },
    state::AppState,
};
use smallvec::SmallVec;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Wry,
};

pub struct Builder {
    lifecycle_manager: Arc<LifecycleManager>,
    service_manager: ServiceManager,
}

impl Builder {
    pub fn new() -> Self {
        let lifecycle_manager = Arc::new(LifecycleManager::new());
        Self {
            lifecycle_manager: Arc::clone(&lifecycle_manager),
            service_manager: ServiceManager::new(lifecycle_manager),
        }
    }
    pub fn with_service(
        self,
        service: impl AnyService + ServiceMetadata,
        activation_points: SmallVec<[ActivationPoint; MAX_ACTIVATION_POINTS]>,
    ) -> Self {
        self.service_manager.register(service, activation_points);

        self
    }

    pub fn build(self) -> TauriPlugin<Wry> {
        PluginBuilder::new("app-formation")
            .setup(move |app_handle, _api| {
                // let state = AppState::new(self.service_manager);

                // app_handle.manage(state);
                // app_handle.manage(self.lifecycle_manager);

                Ok(())
            })
            .build()
    }
}
