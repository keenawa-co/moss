use anyhow::{anyhow, Context as _, Result};
use dashmap::DashSet;
use moss_cache::{backend::moka::MokaBackend, Cache, CacheError};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager};

use crate::{
    app::{service::Service, state::AppState},
    models::application::ThemeDescriptor,
};

const CK_COLOR_THEME: &'static str = "color_theme";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetColorThemeOptions {
    pub enable_cache: bool,
}

pub struct ThemeService {
    app_cache: Arc<Cache<MokaBackend>>,
    themes: Arc<DashSet<ThemeDescriptor>>,
}

impl ThemeService {
    pub fn new(app_handle: &AppHandle) -> Self {
        let app_state = app_handle.state::<AppState>();

        Self {
            app_cache: Arc::clone(&app_state.cache),
            themes: Arc::clone(&app_state.contributions.themes),
        }
    }

    pub fn get_color_themes(&self) -> &DashSet<ThemeDescriptor> {
        &self.themes
    }

    pub async fn get_color_theme(
        &self,
        source: &str,
        opts: Option<GetColorThemeOptions>,
    ) -> Result<String> {
        let handle_cache_miss = || async {
            let content = self.read_color_theme_from_file(source).await?;

            let options = if let Some(options) = opts {
                options
            } else {
                return Ok(content);
            };

            if options.enable_cache {
                self.app_cache.insert(CK_COLOR_THEME, content.clone());
                trace!("Color theme '{}' was successfully cached", source);
            };

            Ok(content)
        };

        match self.app_cache.get::<String>(CK_COLOR_THEME) {
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

impl Service for ThemeService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
