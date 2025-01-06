use crate::{
    addon_registry::AddonRegistry,
    app::{
        service::{AnyService, ServiceMetadata},
        state::AppState,
    },
    models::application::ThemeDescriptor,
};
use anyhow::{anyhow, Result};
use moss_addon::manifest::{AddonManifest, MANIFEST_FILENAME};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tracing::{info, warn};

// use super::{AnyService2, ServiceEvent};

pub struct AddonService {
    addon_registry: AddonRegistry,
    builtin_addons_location: PathBuf,
    installed_addons_location: PathBuf,
}

impl AddonService {
    pub fn new(
        builtin_addons_location: impl Into<PathBuf>,
        installed_addons_location: impl Into<PathBuf>,
    ) -> Self {
        Self {
            addon_registry: AddonRegistry::new(),
            builtin_addons_location: builtin_addons_location.into(),
            installed_addons_location: installed_addons_location.into(),
        }
    }

    fn parse_addon_dir(&self, app_state: &AppState, addon_dir: PathBuf) -> Result<()> {
        let read_dir = std::fs::read_dir(&addon_dir)
            .map_err(|err| anyhow!("Failed to read the directory {:?}: {}", addon_dir, err))?;

        for entry_result in read_dir {
            let Ok(entry) = entry_result else {
                info!(
                    "Failed to read an entry in the directory of addon: {:?}",
                    entry_result.err()
                );
                continue;
            };

            let entry_path = entry.path();

            if !entry_path.is_dir() && entry.file_name() == MANIFEST_FILENAME {
                let file_content = std::fs::read_to_string(&entry_path)
                    .map_err(|err| anyhow!("Failed to read the {:?} file: {}", entry_path, err))?;

                let addon_manifest: AddonManifest = toml::from_str(&file_content)
                    .map_err(|err| anyhow!("Failed to parse the {:?} file: {}", entry_path, err))?;

                if let Some(themes) = addon_manifest.contributes.themes {
                    for theme_contribution in themes {
                        app_state.contributions.themes.insert(ThemeDescriptor {
                            id: format!(
                                "{}.{}",
                                addon_manifest.addon.name,
                                theme_contribution.label.replace(" ", "")
                            ),
                            name: theme_contribution.label,
                            source: addon_dir
                                .join(theme_contribution.path)
                                .to_string_lossy()
                                .to_string(),
                        });
                    }
                }
            }
        }

        Ok(())
    }
}

impl AnyService for AddonService {
    fn start(&self, app_handle: &AppHandle) {
        let app_state = app_handle.state::<AppState>();

        let mut read_dir = std::fs::read_dir(&self.builtin_addons_location).unwrap_or_else(|err| {
            panic!(
                "Failed to read the directory {:?} containing built-in addons: {err}",
                &self.builtin_addons_location
            );
        });

        for entry_result in read_dir {
            let Ok(entry) = entry_result else {
                warn!(
                    "Failed to read an entry in the directory for built-in addons: {:?}",
                    entry_result.err()
                );
                continue;
            };

            if !entry.path().is_dir() {
                continue;
            }

            if let Err(err) = self.parse_addon_dir(&app_state, entry.path()) {
                warn!("Failed to parse addon: {err}");
                continue;
            };
        }
    }

    fn stop(&self, _app_handle: &AppHandle) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ServiceMetadata for AddonService {
    const SERVICE_BRAND: &'static str = "AddonService";
}
