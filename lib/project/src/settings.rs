use anyhow::Result;
use async_utl::AsyncTryFrom;
use hashbrown::HashSet;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{watch, RwLock};
use types::file::json_file::JsonFile;

#[derive(Debug, Deserialize)]
pub(crate) struct SettingsFileRepresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project.monitoring.exclude")]
    exclude_list: Option<Vec<String>>,
}

#[derive(Debug)]
struct MonitoringExcludeList {
    cache: RwLock<Vec<String>>,
    watch_tx: watch::Sender<HashSet<PathBuf>>,
    watch_rx: watch::Receiver<HashSet<PathBuf>>,
}

#[derive(Debug)]
pub struct Settings {
    file: Arc<JsonFile>,
    monitoring_exclude: MonitoringExcludeList,
}

impl Settings {
    pub fn watch_monitoring_exclude_list(&self) -> watch::Receiver<HashSet<PathBuf>> {
        self.monitoring_exclude.watch_rx.clone()
    }

    pub async fn append_to_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        let input_set: HashSet<PathBuf> = input_list.iter().cloned().collect();
        let mut current_exclude_list = self.fetch_exclude_list().await;
        let additional_exclusion_list: HashSet<PathBuf> = input_set
            .difference(&current_exclude_list)
            .cloned()
            .collect();
        if additional_exclusion_list.is_empty() {
            return Ok(self.monitoring_exclude.cache.read().await.clone());
        }

        {
            let mut exclude_cache_list = self.monitoring_exclude.cache.write().await;

            current_exclude_list.extend(additional_exclusion_list.iter().cloned());
            exclude_cache_list.extend(
                additional_exclusion_list
                    .iter()
                    .map(|item| item.to_string_lossy().to_string())
                    .collect::<Vec<String>>(),
            );

            self.file
                .write_by_path("/project.monitoring.exclude", &*exclude_cache_list)
                .await?;
        }

        self.monitoring_exclude
            .watch_tx
            .send(current_exclude_list.clone())?;

        Ok(self.monitoring_exclude.cache.read().await.clone())
    }

    pub async fn fetch_exclude_list(&self) -> HashSet<PathBuf> {
        self.monitoring_exclude.watch_rx.borrow().clone().into()
    }

    pub async fn remove_from_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        let should_be_removed = input_list.iter().cloned().collect::<HashSet<PathBuf>>();
        if should_be_removed.is_empty() {
            return Ok(self.monitoring_exclude.cache.read().await.to_vec());
        }

        let mut current_exclude_list = self.fetch_exclude_list().await;
        if current_exclude_list.is_empty() {
            return Ok(vec![]);
        }

        let should_be_removed_as_string = should_be_removed
            .iter()
            .map(|item| item.to_string_lossy().to_string())
            .collect::<HashSet<String>>();

        {
            let mut exclude_cache_list = self.monitoring_exclude.cache.write().await;

            current_exclude_list.retain(|item| !should_be_removed.contains(item));
            exclude_cache_list.retain(|item| !should_be_removed_as_string.contains(item));

            self.file
                .write_by_path("/project.monitoring.exclude", &*exclude_cache_list)
                .await?;
        }

        self.monitoring_exclude
            .watch_tx
            .send(current_exclude_list.clone())?;

        Ok(current_exclude_list
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect())
    }
}

#[async_trait]
impl AsyncTryFrom<Arc<JsonFile>> for Settings {
    type Error = anyhow::Error;

    async fn try_from_async(file: Arc<JsonFile>) -> Result<Self, Self::Error> {
        let settings_file = file
            .get_by_path::<SettingsFileRepresentation>("/")
            .await?
            .ok_or_else(|| anyhow!("Module settings not found"))?;

        let initial_exclude_list: HashSet<PathBuf> = settings_file
            .exclude_list
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(PathBuf::from)
            .collect();

        let monitoring_exclude = {
            let (tx, rx) = watch::channel(HashSet::new());

            MonitoringExcludeList {
                cache: RwLock::new(
                    initial_exclude_list
                        .iter()
                        .map(|path| path.to_string_lossy().to_string())
                        .collect(),
                ),
                watch_tx: tx,
                watch_rx: rx,
            }
        };

        let settings = Self {
            file,
            monitoring_exclude,
        };

        settings
            .monitoring_exclude
            .watch_tx
            .send(initial_exclude_list)?;

        Ok(settings)
    }
}
