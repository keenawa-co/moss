use anyhow::{anyhow, Result};
use moss_addon::manifest::{AddonManifest, MANIFEST_FILENAME};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::{
    addon_registry::AddonRegistry,
    app::{
        service::{AnyService, ServiceMetadata},
        state::AppState,
    },
    models::application::ThemeDescriptor,
};

pub struct AddonService {
    addon_registry: AddonRegistry,
}

impl AddonService {
    pub fn new(
        app_handle: &AppHandle,
        builtin_addons_location: impl Into<PathBuf>,
        installed_addons_location: impl Into<PathBuf>,
    ) -> Self {
        let builtin_addons_location = builtin_addons_location.into();
        let _installed_addons_location = installed_addons_location.into();

        let mut read_dir = std::fs::read_dir(&builtin_addons_location).unwrap_or_else(|err| {
            panic!(
                "Failed to read the directory {:?} containing built-in addons: {err}",
                &builtin_addons_location
            );
        });

        let app_state = app_handle.state::<AppState>();
        while let Some(entry) = read_dir.next() {
            let Ok(entry) = entry else {
                warn!(
                    "Failed to read an entry in the directory for built-in addons: {:?}",
                    entry.err()
                );
                continue;
            };

            if !entry.path().is_dir() {
                continue;
            }

            if let Err(err) = parse_addon_dir(&app_state, entry.path()) {
                warn!("Failed to parse addon: {err}");
                continue;
            };
        }

        Self {
            addon_registry: AddonRegistry::new(),
        }
    }
}

impl AnyService for AddonService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn parse_addon_dir(app_state: &AppState, addon_dir: PathBuf) -> Result<()> {
    let mut read_dir = std::fs::read_dir(&addon_dir)
        .map_err(|err| anyhow!("Failed to read the directory {:?}: {}", addon_dir, err))?;

    while let Some(entry) = read_dir.next() {
        let Ok(entry) = entry else {
            info!(
                "Failed to read an entry in the directory of addon: {:?}",
                entry.err()
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

impl ServiceMetadata for AddonService {}
