use crate::{
    app::{service::Service, state::AppState},
    models::application::LocaleDescriptor,
};
use anyhow::{anyhow, Context as _, Result};
use dashmap::DashSet;
use dirs::home_dir;
use moss_cache::{backend::moka::MokaBackend, Cache, CacheError};
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager};

const CK_TRANSLATIONS: &'static str = "translations";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTranslationsOptions {
    pub enable_cache: bool,
}

pub struct LocaleService {
    app_cache: Arc<Cache<MokaBackend>>,
    locales: Arc<DashSet<LocaleDescriptor>>,
}

impl LocaleService {
    pub fn new(app_handle: &AppHandle) -> Self {
        let app_state = app_handle.state::<AppState>();

        Self {
            app_cache: Arc::clone(&app_state.cache),
            locales: Arc::clone(&app_state.contributions.locales),
        }
    }

    pub fn get_locales(&self) -> &DashSet<LocaleDescriptor> {
        &self.locales
    }
    pub async fn get_translations(
        &self,
        language: &str,
        namespace: &str,
        opts: Option<GetTranslationsOptions>,
    ) -> Result<Value> {
        let handle_cache_miss = || async {
            let content = self
                .read_translations_from_file(language, namespace)
                .await?;
            let options = if let Some(options) = opts {
                options
            } else {
                return Ok(content);
            };

            if options.enable_cache {
                self.app_cache
                    .insert(&format!("{CK_TRANSLATIONS}-{namespace}"), content.clone());
                trace!("Language pack '{language}-{namespace}' was successfully cached");
            };
            Ok(content)
        };

        match self
            .app_cache
            .get::<Value>(&format!("{CK_TRANSLATIONS}-{namespace}"))
        {
            Ok(cached_value) => {
                trace!("Language pack '{language}-{namespace}' was restored from the cache");

                Ok((*cached_value).clone())
            }
            Err(CacheError::NonexistentKey { .. }) => handle_cache_miss().await,
            Err(CacheError::TypeMismatch { key, type_name }) => {
                warn!(
                    "Type mismatch for key '{}': expected 'Value', found '{}'",
                    key, type_name
                );

                handle_cache_miss().await
            }
        }
    }

    async fn read_translations_from_file(&self, language: &str, namespace: &str) -> Result<Value> {
        let locales_dir = get_locales_dir()?;
        let full_path = locales_dir.join(language).join(format!("{namespace}.json"));

        if !full_path.exists() {
            return Err(anyhow!("File '{}' does not exist", full_path.display()));
        }

        if !full_path.is_file() {
            return Err(anyhow!("Path '{}' is not a file", full_path.display()));
        }

        let content = smol::fs::read_to_string(&full_path)
            .await
            .with_context(|| format!("Failed to read file '{}'", full_path.display()))?;

        serde_json::from_str::<Value>(&content)
            .with_context(|| format!("Failed to parse file '{}'", full_path.display()))
    }
}

impl Service for LocaleService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn get_locales_dir() -> Result<PathBuf> {
    Ok(home_dir()
        .ok_or(anyhow!("Couldn't get the home directory"))?
        .join(".config")
        .join("moss")
        .join("locales"))
}
