use anyhow::{anyhow, Context as _, Result};
use moss_cache::{backend::moka::MokaBackend, Cache, CacheError};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use tauri::AppHandle;

const CK_COLOR_THEME: &'static str = "color_theme";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetColorThemeOptions {
    pub enable_cache: bool,
}

pub struct ThemeService {
    app_handle: AppHandle,
    app_cache: Arc<Cache<MokaBackend>>,
}

impl ThemeService {
    pub fn new(app_handle: AppHandle, app_cache: Arc<Cache<MokaBackend>>) -> Self {
        Self {
            app_handle,
            app_cache,
        }
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

fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("Home directory not found!")
}

fn get_themes_dir() -> Result<PathBuf> {
    Ok(get_home_dir()?.join(".config").join("moss").join("themes"))
}
