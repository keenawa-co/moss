use anyhow::Result;
use dashmap::DashMap;
use desktop_models::appearance::theming::ThemeDescriptor;
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tauri::{AppHandle, Window};
use workbench_desktop::window::NativePlatformInfo;
use workbench_desktop::Workbench;

// NOTE: Temporary solution. Will be moved to crates/moss-desktop.

pub struct Appearance {
    pub theme_descriptor: RwLock<ThemeDescriptor>,
}

impl Appearance {
    pub fn set_theme_descriptor(&self, new_theme_descriptor: ThemeDescriptor) {
        let mut theme_descriptor_lock = self.theme_descriptor.write();
        *theme_descriptor_lock = new_theme_descriptor;
    }
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
    pub next_window_id: AtomicUsize,

    pub commands: DashMap<ReadOnlyStr, CommandHandler>,

    pub workbench: Arc<Workbench>,
    pub platform_info: NativePlatformInfo,
}

impl AppState {
    pub fn get_command(&self, id: &ReadOnlyStr) -> Option<CommandHandler> {
        self.commands.get(id).map(|cmd| Arc::clone(&cmd))
    }
}
