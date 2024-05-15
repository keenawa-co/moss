use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use types::asynx::AsyncTryFrom;
use workspace::settings::{FileAdapter, Settings};

#[derive(Debug)]
pub struct ProjectSettings {
    settings_file: Arc<FileAdapter>,
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
    ) -> anyhow::Result<Vec<String>> {
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
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<Vec<String>> {
        let path_value = path.as_ref().to_string_lossy().into_owned();

        self.settings_file
            .remove_from_array_fragment("$.['project.monitoring.exclude']", &path_value)
            .await?;

        let mut module = self.module_settings.write().await;
        if let Some(exclude) = &mut module.monitoring.exclude {
            exclude.retain(|item| item != &path_value);
        }

        Ok(module.monitoring.exclude.clone().unwrap_or_default())
    }
}

#[async_trait]
impl AsyncTryFrom<Arc<workspace::settings::FileAdapter>> for ProjectSettings {
    type Error = anyhow::Error;

    async fn try_from_async(
        value: Arc<workspace::settings::FileAdapter>,
    ) -> Result<Self, Self::Error> {
        let module_settings = value.get_by_path("$").await?;

        Ok(Self {
            settings_file: value.clone(),
            module_settings: Arc::new(RwLock::new(module_settings)),
        })
    }
}
