use anyhow::{anyhow, Context as _, Result};
use dashmap::DashSet;
use moss_cache::{backend::moka::MokaBackend, Cache, CacheError};
use moss_theme::{
    conversion::{
        json_converter::JsonThemeConverter, jsonschema_validator::JsonSchemaValidator,
        ThemeConverter,
    },
    schema::SCHEMA_THEME,
};
use std::{ops::Deref, path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager};

use crate::{
    app::{service::Service, state::AppStateManager},
    models::application::ThemeDescriptor,
};

const CK_COLOR_THEME: &str = "color_theme";

#[derive(Clone)]
struct ThemeCacheEntry {
    source: String,
    data: String,
}

pub struct ThemeService {
    app_cache: Arc<Cache<MokaBackend>>,
    converter: Arc<dyn ThemeConverter + Send + Sync>,
    themes: Arc<DashSet<ThemeDescriptor>>,
}

impl ThemeService {
    pub fn new(app_handle: &AppHandle) -> Self {
        let app_state = app_handle.state::<AppStateManager>();
        let json_schema_validator = JsonSchemaValidator::new(SCHEMA_THEME.deref());
        let converter = JsonThemeConverter::new(json_schema_validator);

        // let s = SCHEMA_THEME.deref();

        Self {
            app_cache: Arc::clone(&app_state.cache),
            converter: Arc::new(converter),
            themes: Arc::clone(&app_state.contributions.themes),
        }
    }

    pub fn get_color_themes(&self) -> &DashSet<ThemeDescriptor> {
        &self.themes
    }

    pub async fn get_color_theme(&self, source: &str) -> Result<String> {
        match self.app_cache.get::<ThemeCacheEntry>(CK_COLOR_THEME) {
            Ok(entry) if entry.source == source => {
                trace!("Color theme '{}' was restored from the cache", source);
                return Ok(entry.data.clone());
            }
            Ok(_) => {
                trace!(
                    "Color theme in cache does not match the requested source '{}'",
                    source
                );
            }
            Err(CacheError::NonexistentKey { .. }) => {
                trace!("No color theme found in cache for key '{}'", CK_COLOR_THEME);
            }
            Err(CacheError::TypeMismatch { key, type_name }) => {
                warn!(
                    "Type mismatch for key '{}': expected 'ThemeCacheEntry', found '{}'",
                    key, type_name
                );
            }
        }

        self.get_color_theme_internal(source).await
    }

    async fn get_color_theme_internal(&self, source: &str) -> Result<String> {
        let json_data = self.read_color_theme_from_file(source).await?;
        // TODO: Add merging of the global theme object with the user’s custom theme settings object.
        let css_data = self.converter.convert_to_css(json_data)?;

        self.app_cache.insert(
            CK_COLOR_THEME,
            ThemeCacheEntry {
                source: source.to_string(),
                data: css_data.clone(),
            },
        );

        Ok(css_data)
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
