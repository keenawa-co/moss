use moss_desktop::{
    services::{ActivationPoint, AnyService, ServiceManager, MAX_ACTIVATION_POINTS},
    state::AppState,
};
use smallvec::SmallVec;
use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Wry,
};

pub struct Builder {
    service_manager: ServiceManager,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            service_manager: ServiceManager::new(),
        }
    }
    pub fn with_service(
        self,
        service: impl AnyService,
        activation_points: SmallVec<[ActivationPoint; MAX_ACTIVATION_POINTS]>,
    ) -> Self {
        self.service_manager.register(service, activation_points);

        self
    }

    pub fn build(self) -> TauriPlugin<Wry> {
        PluginBuilder::new("app-formation")
            .setup(move |app_handle, _api| {
                let state = AppState::new();

                self.service_manager.run(app_handle.clone());

                app_handle.manage(state);
                app_handle.manage(self.service_manager);

                dbg!("app-formation");
                Ok(())
            })
            .build()
    }
}
