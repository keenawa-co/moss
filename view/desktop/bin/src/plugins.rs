#[cfg(target_os = "macos")]
pub mod mac_window;

use tauri::plugin::TauriPlugin;
use tauri::Runtime;

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
            true
        }
        #[cfg(not(dev))]
        {
            false
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
