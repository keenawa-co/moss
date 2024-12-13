use anyhow::Result;
use dashmap::DashMap;
use desktop_models::appearance::theming::ThemeDescriptor;
use desktop_models::window::LocaleDescriptor;
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tauri::{AppHandle, Window};
use workbench_desktop::Workbench;

// NOTE: Temporary solution. Will be moved to crates/moss-desktop.

pub struct Appearance {
    pub theme: RwLock<ThemeDescriptor>,
}

pub struct CommandContext {
    pub app_handle: AppHandle,
    pub window: Window,
    args: HashMap<String, Value>,
}

impl CommandContext {
    pub fn new(app_handle: AppHandle, window: Window, args: HashMap<String, Value>) -> Self {
        Self {
            app_handle,
            window,
            args,
        }
    }

    pub fn get_arg<T>(&self, key: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let value = self
            .args
            .get(key)
            .ok_or_else(|| format!("Argument '{}' is not found", key))?;
        serde_json::from_value(value.clone())
            .map_err(|e| format!("Deserialization error for key'{}': {}", key, e))
    }
}

pub type CommandHandler =
    Arc<dyn Fn(CommandContext, &AppState) -> Result<Value, String> + Send + Sync>;

pub struct AppState {
    pub appearance: Appearance,
    pub locale: RwLock<LocaleDescriptor>,
    pub next_window_id: AtomicUsize,
    pub commands: DashMap<ReadOnlyStr, CommandHandler>,
    pub workbench: Arc<Workbench>,
}

impl AppState {
    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.commands.get(id).map(|cmd| Arc::clone(&cmd))
    }

    pub fn change_language_pack(&self, locale_descriptor: LocaleDescriptor) {
        let mut locale_lock = self.locale.write();
        *locale_lock = locale_descriptor;
    }

    pub fn change_color_theme(&self, new_theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.appearance.theme.write();
        *theme_descriptor_lock = new_theme_descriptor;
    }
}
