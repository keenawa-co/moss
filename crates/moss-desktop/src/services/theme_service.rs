use anyhow::{anyhow, Context as _, Result};
use dashmap::DashSet;
use moss_cache::{backend::moka::MokaBackend, Cache, CacheError};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager};

use crate::{
    app::{
        service::{AnyService, ServiceMetadata},
        state::AppState,
    },
    models::application::ThemeDescriptor,
};

const CK_COLOR_THEME: &'static str = "color_theme";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetColorThemeOptions {
    pub enable_cache: bool,
}

pub struct ThemeService {
    app_cache: OnceCell<Arc<Cache<MokaBackend>>>,
    themes: OnceCell<Arc<DashSet<ThemeDescriptor>>>,
}

impl ThemeService {
    pub fn new() -> Self {
        Self {
            app_cache: OnceCell::new(),
            themes: OnceCell::new(),
        }
    }

    pub fn get_color_themes(&self) -> &DashSet<ThemeDescriptor> {
        self.themes.get().unwrap()
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

    async fn read_color_theme_from_file(&self, path: &str) -> Result<String> {
        let full_path = PathBuf::from(path);

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

impl AnyService for ThemeService {
    fn start(&self, app_handle: &AppHandle) {
        let app_state = app_handle.state::<AppState>();

        self.themes
            .set(app_state.contributions.themes.clone())
            .unwrap();

        self.app_cache
        .set(Arc::clone(&app_state.cache))
        .unwrap_or_else(|_| {
            panic!("Failed to set the app cache in ThemeService: the cache has already been initialized.")
        });
    }

    fn stop(&self, _app_handle: &AppHandle) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ServiceMetadata for ThemeService {}
