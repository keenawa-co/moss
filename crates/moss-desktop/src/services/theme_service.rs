use anyhow::{anyhow, Context as _, Result};
use async_trait::async_trait;
use moss_cache::{backend::moka::MokaBackend, Cache, CacheError};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tauri::Manager;

use crate::app::state::AppState;

use super::{AnyService, ServiceEvent};

const CK_COLOR_THEME: &'static str = "color_theme";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetColorThemeOptions {
    pub enable_cache: bool,
}

pub struct ThemeService {
    app_cache: OnceCell<Arc<Cache<MokaBackend>>>,
}

impl ThemeService {
    pub fn new() -> Self {
        Self {
            app_cache: OnceCell::new(),
        }
    }

    pub async fn get_color_theme(
        &self,
        source: &str,
        opts: Option<GetColorThemeOptions>,
    ) -> Result<String> {
        let app_cache = self.app_cache.get().unwrap();

        let handle_cache_miss = || async {
            let content = self.read_color_theme_from_file(source).await?;

            let options = if let Some(options) = opts {
                options
            } else {
                return Ok(content);
            };

            if options.enable_cache {
                app_cache.insert(CK_COLOR_THEME, content.clone());
                trace!("Color theme '{}' was successfully cached", source);
            };

            Ok(content)
        };

        match app_cache.get::<String>(CK_COLOR_THEME) {
            Ok(cached_value) => {
                trace!("Color theme '{source}' was restored from the cache");

                Ok((*cached_value).clone())
            }
            Err(CacheError::NonexistentKey { .. }) => handle_cache_miss().await,
            Err(CacheError::TypeMismatch { key, type_name }) => {
                warn!(
                    "Type mismatch for key '{}': expected 'String', found '{}'",
                    key, type_name
                );

                handle_cache_miss().await
            }
        }
    }

    async fn read_color_theme_from_file(&self, source: &str) -> Result<String> {
        let themes_dir = get_themes_dir().context("Failed to get the themes directory")?;
        let full_path = themes_dir.join(source);

        if !full_path.exists() {
            return Err(anyhow!("File '{}' does not exist", full_path.display()));
        }

        if !full_path.is_file() {
            return Err(anyhow!("Path '{}' is not a file", full_path.display()));
        }

        let content = smol::fs::read_to_string(&full_path)
            .await
            .with_context(|| format!("Failed to read file '{}'", full_path.display()))?;

        Ok(content)
    }
}

impl ThemeService {
    fn on_activation(&self, app_state: &AppState) {
        self.app_cache
            .set(Arc::clone(&app_state.cache))
            .unwrap_or_else(|_| {
                panic!("Failed to set the app cache in ThemeService: the cache has already been initialized.")
            });
    }
}

#[async_trait]
impl AnyService for ThemeService {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn on_event(&self, app_handle: tauri::AppHandle, event: ServiceEvent) {
        let app_state = app_handle.state::<AppState>();

        match event {
            ServiceEvent::Activation => self.on_activation(&app_state),
        }
    }
}

fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("Home directory not found!")
}

fn get_themes_dir() -> Result<PathBuf> {
    Ok(get_home_dir()?.join(".config").join("moss").join("themes"))
}
