pub mod app_formation;
pub mod mac_window;

use tauri::plugin::TauriPlugin;
use tauri::{Runtime, Wry};

pub mod plugin_log {
    use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};

    use super::*;

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        tauri_plugin_log::Builder::default()
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: None }),
                Target::new(TargetKind::Webview),
            ])
            .level_for("tao", log::LevelFilter::Info)
            .level_for("plugin_runtime", log::LevelFilter::Info)
            .level_for("tracing", log::LevelFilter::Warn)
            .with_colors(ColoredLevelConfig::default())
            .level(if is_dev() {
                log::LevelFilter::Trace
            } else {
                log::LevelFilter::Info
            })
            .build()
    }

    fn is_dev() -> bool {
        #[cfg(dev)]
        {
            return true;
        }
        #[cfg(not(dev))]
        {
            return false;
        }
    }
}

pub mod plugin_window_state {
    use super::*;

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        tauri_plugin_window_state::Builder::default()
            .with_denylist(&["ignored"])
            .map_label(|label| {
                if label.starts_with(crate::OTHER_WINDOW_PREFIX) {
                    "ignored"
                } else {
                    label
                }
            })
            .build()
    }
}

pub mod plugin_app_formation {
    use moss_desktop::{
        app::service::ActivationPoint,
        services::{addon_service::AddonService, theme_service::ThemeService},
    };
    use smallvec::smallvec;
    use std::path::PathBuf;

    use super::*;

    pub fn init() -> TauriPlugin<Wry> {
        app_formation::Builder::new()
            .with_service(
                AddonService::new(builtin_addons_dir(), installed_addons_dir()),
                smallvec![ActivationPoint::OnBootstrapping],
            )
            .with_service(
                ThemeService::new(),
                smallvec![ActivationPoint::OnBootstrapping],
            )
            .build()
    }

    fn builtin_addons_dir() -> impl Into<PathBuf> {
        let workspace_root =
            std::env::current_dir().expect("Failed to retrieve the current working directory");

        dbg!(&workspace_root);

        std::env::var("BUILTIN_ADDONS_DIR")
            .expect("Environment variable `BUILTIN_ADDONS_DIR` is not set or is invalid")
    }

    fn installed_addons_dir() -> impl Into<PathBuf> {
        std::env::var("INSTALLED_ADDONS_DIR")
            .expect("Environment variable `INSTALLED_ADDONS_DIR` is not set or is invalid")
    }
}
