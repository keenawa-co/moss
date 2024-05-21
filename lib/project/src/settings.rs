use anyhow::Result;
use async_utl::AsyncTryFrom;
use hashbrown::HashSet;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{watch, RwLock};
use types::file::json_file::JsonFile;

#[derive(Debug, Deserialize)]
pub(crate) struct SettingsFile {
    #[serde(rename = "project.monitoring.exclude")]
    #[serde(default = "SettingsFile::default_exclude_list")]
    monitoring_exclude_list: Vec<String>,

    #[serde(rename = "project.worktree.preference.displayExcludedEntries")]
    #[serde(default = "SettingsFile::default_display_excluded_entries")]
    display_excluded_entries: bool,

    #[serde(rename = "project.worktree.preference.displayGitIgnoredEntries")]
    #[serde(default = "SettingsFile::default_display_gitignore_entries")]
    display_gitignore_entries: bool,

    #[serde(rename = "project.monitoring.watchGitIgnoredEntries")]
    #[serde(default = "SettingsFile::default_watch_gitignore_entries")]
    watch_gitignore_entries: bool,

    #[serde(rename = "project.monitoring.autoWatchNewEntries")]
    #[serde(default = "SettingsFile::default_auto_watch_new")]
    auto_watch_new_entries: bool,
}

impl SettingsFile {
    fn default_exclude_list() -> Vec<String> {
        vec![]
    }

    fn default_display_excluded_entries() -> bool {
        false
    }

    fn default_display_gitignore_entries() -> bool {
        true
    }

    fn default_watch_gitignore_entries() -> bool {
        true
    }

    fn default_auto_watch_new() -> bool {
        true
    }
}

#[derive(Debug)]
// TODO: use glob::Pattern
struct MonitoringExcludeList {
    cache: RwLock<Vec<String>>,
    watch_tx: watch::Sender<HashSet<PathBuf>>,
    watch_rx: watch::Receiver<HashSet<PathBuf>>,
}

impl Into<HashSet<PathBuf>> for MonitoringExcludeList {
    fn into(self) -> HashSet<PathBuf> {
        self.watch_rx.borrow().clone().into()
    }
}

#[derive(Debug)]
pub struct Settings {
    pub(crate) file: Arc<JsonFile>,
    pub(crate) monitoring_exclude_list: MonitoringExcludeList,
    pub(crate) display_excluded_entries: bool,
    pub(crate) display_gitignore_entries: bool,
    pub(crate) watch_gitignore_entries: bool,
    pub(crate) auto_watch_new_entries: bool,
}

impl Settings {
    pub fn watch_monitoring_exclude_list(&self) -> watch::Receiver<HashSet<PathBuf>> {
        self.monitoring_exclude_list.watch_rx.clone()
    }

    pub async fn append_to_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        let input_set: HashSet<PathBuf> = input_list.iter().cloned().collect();
        let mut current_exclude_list = self.fetch_exclude_list();
        let additional_exclusion_list: HashSet<PathBuf> = input_set
            .difference(&current_exclude_list)
            .cloned()
            .collect();
        if additional_exclusion_list.is_empty() {
            return Ok(self.monitoring_exclude_list.cache.read().await.clone());
        }

        {
            let mut exclude_cache_list = self.monitoring_exclude_list.cache.write().await;

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

        self.monitoring_exclude_list
            .watch_tx
            .send(current_exclude_list.clone())?;

        Ok(self.monitoring_exclude_list.cache.read().await.clone())
    }

    pub fn fetch_exclude_list(&self) -> HashSet<PathBuf> {
        self.monitoring_exclude_list
            .watch_rx
            .borrow()
            .clone()
            .into()
    }

    pub async fn remove_from_monitoring_exclude_list(
        &self,
        input_list: &[PathBuf],
    ) -> Result<Vec<String>> {
        let should_be_removed = input_list.iter().cloned().collect::<HashSet<PathBuf>>();
        if should_be_removed.is_empty() {
            return Ok(self.monitoring_exclude_list.cache.read().await.to_vec());
        }

        let mut current_exclude_list = self.fetch_exclude_list();
        if current_exclude_list.is_empty() {
            return Ok(vec![]);
        }

        let should_be_removed_as_string = should_be_removed
            .iter()
            .map(|item| item.to_string_lossy().to_string())
            .collect::<HashSet<String>>();

        {
            let mut exclude_cache_list = self.monitoring_exclude_list.cache.write().await;

            current_exclude_list.retain(|item| !should_be_removed.contains(item));
            exclude_cache_list.retain(|item| !should_be_removed_as_string.contains(item));

            self.file
                .write_by_path("/project.monitoring.exclude", &*exclude_cache_list)
                .await?;
        }

        self.monitoring_exclude_list
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
            .get_by_path::<SettingsFile>("/")
            .await?
            .ok_or_else(|| anyhow!("Module settings not found"))?;

        let initial_exclude_list: HashSet<PathBuf> = settings_file
            .monitoring_exclude_list
            .into_iter()
            .map(PathBuf::from)
            .collect();

        let monitoring_exclude_list = {
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
            monitoring_exclude_list,
            display_excluded_entries: settings_file.display_excluded_entries,
            display_gitignore_entries: settings_file.display_gitignore_entries,
            watch_gitignore_entries: settings_file.watch_gitignore_entries,
            auto_watch_new_entries: settings_file.auto_watch_new_entries,
        };

        settings
            .monitoring_exclude_list
            .watch_tx
            .send(initial_exclude_list)?;

        Ok(settings)
    }
}
