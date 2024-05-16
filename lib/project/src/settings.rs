use anyhow::{Error, Result};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use types::asynx::AsyncTryFrom;
use workspace::settings::SettingsFile;

#[derive(Debug)]
pub struct ProjectSettings {
    settings_file: Arc<SettingsFile>,
    module_settings: Arc<RwLock<Module>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Module {
    #[serde(flatten)]
    pub monitoring: Monitoring,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitoring {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project.monitoring.exclude")]
    pub exclude: Option<Vec<String>>,
}

impl ProjectSettings {
    pub async fn append_to_monitoring_exclude_list(
        &self,
        exclude_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        let mut module_lock = self.module_settings.write().await;
        let mut new_exclude_list = module_lock
            .monitoring
            .exclude
            .clone()
            .unwrap_or_else(Vec::new);

        let existing: hashbrown::HashSet<String> = new_exclude_list.iter().cloned().collect();
        let new_items: Vec<String> = exclude_list
            .iter()
            .map(|item| item.to_string_lossy().into_owned())
            .filter(|item| !existing.contains(item))
            .collect();

        if new_items.is_empty() {
            return Ok(new_exclude_list);
        }

        new_exclude_list.extend(new_items);

        self.settings_file
            .write_by_path("/project.monitoring.exclude", &new_exclude_list)
            .await?;

        module_lock.monitoring.exclude = Some(new_exclude_list.clone());

        Ok(new_exclude_list)
    }

    pub async fn fetch_exclude_list(&self) -> Option<Vec<String>> {
        let module_lock = self.module_settings.read().await;

        module_lock.monitoring.exclude.clone()
    }

    pub async fn remove_from_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        let mut module_lock = self.module_settings.write().await;
        let exclude_list = module_lock.monitoring.exclude.get_or_insert_with(Vec::new);
        if exclude_list.is_empty() {
            return Ok(vec![]);
        }

        let should_be_removed: hashbrown::HashSet<String> = input_list
            .iter()
            .map(|item| item.to_string_lossy().to_string())
            .collect();

        exclude_list.retain(|item| !should_be_removed.contains(item));

        self.settings_file
            .write_by_path("/project.monitoring.exclude", &exclude_list)
            .await?;

        Ok(exclude_list.clone())
    }
}

#[async_trait]
impl AsyncTryFrom<Arc<workspace::settings::SettingsFile>> for ProjectSettings {
    type Error = anyhow::Error;

    async fn try_from_async(
        value: Arc<workspace::settings::SettingsFile>,
    ) -> Result<Self, Self::Error> {
        let module_settings = value
            .get_by_path("/")
            .await?
            .ok_or_else(|| anyhow!("Module settings not found"))?;

        Ok(Self {
            settings_file: value.clone(),
            module_settings: Arc::new(RwLock::new(module_settings)),
        })
    }
}
