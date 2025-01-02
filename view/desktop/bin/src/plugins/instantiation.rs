// use moss_desktop::app::{
//     instantiation::{InstantiationType, ServiceCollection},
//     service::AnyService,
//     state::StateManager,
// };
// use tauri::{
//     plugin::{Builder as PluginBuilder, TauriPlugin},
//     Manager, Wry,
// };

// pub struct Builder {
//     services: ServiceCollection,
// }

// impl Builder {
//     // pub fn new() -> Self {
//     //     let r = PluginBuilder::new("app-formation");
//     //     Self {
//     //         services: ServiceCollection::new(app_handle),
//     //     }
//     // }

//     pub fn service<T: AnyService>(mut self, service: T, activation_type: InstantiationType) {
//         self.services.register(service, activation_type);
//     }

//     pub fn build(self) {
//         // let state = StateManager::new(self.services);

//         // app_handle.manage(state);
//         // app_handle.manage(self.lifecycle_manager);
//     }
// }
