use anyhow::Result;
use async_trait::async_trait;
use moss_addon::manifest::MANIFEST_FILENAME;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::{
    addon_registry::AddonRegistry,
    app::{service::AnyService, state::AppState, subscription::Subscription},
};

// use super::{AnyService2, ServiceEvent};

pub struct AddonService {
    addons: AddonRegistry,
    builtin_addons_location: PathBuf,
    installed_addons_location: PathBuf,
    lifecycle_manager_sub: Subscription,
}

impl AddonService {
    pub fn new(
        builtin_addons_location: impl Into<PathBuf>,
        installed_addons_location: impl Into<PathBuf>,
        lifecycle_manager_sub: Subscription,
    ) -> Self {
        Self {
            addons: AddonRegistry::new(),
            builtin_addons_location: builtin_addons_location.into(),
            installed_addons_location: installed_addons_location.into(),
            lifecycle_manager_sub,
        }
    }

    pub async fn on_activation(&self, app_state: &AppState) -> Result<()> {
        let mut read_dir = tokio::fs::read_dir(&self.builtin_addons_location).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            if entry.path().is_dir() {
                continue;
            }

            if entry.file_name() == MANIFEST_FILENAME {
                dbg!(entry.file_name());
            }
        }

        Ok(())
    }
}

// #[async_trait]
// impl AnyService2 for AddonService {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     async fn on_event(&self, app_handle: tauri::AppHandle, event: ServiceEvent) {
//         // let state = app_handle.state::<AppState>();
//         match event {
//             ServiceEvent::Activation => {
//                 dbg!("on_event!");
//                 //
//                 // self.on_activation(&state).await.unwrap();
//             }
//         }
//     }
// }

impl AnyService for AddonService {
    fn start(&self, app_handle: &AppHandle) {
        todo!()
    }

    fn stop(&self, app_handle: &AppHandle) {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}
