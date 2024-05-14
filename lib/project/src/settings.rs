use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use types::asynx::AsyncTryFrom;

#[derive(Debug)]
pub struct Settings {
    workspace: Arc<workspace::settings::Settings>,
    module: Arc<RwLock<Module>>,
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

impl Settings {
    pub async fn append_to_monitoring_exclude_list(
        &self,
        exclude_list: &[PathBuf],
    ) -> anyhow::Result<Vec<String>> {
        self.workspace
            .append_to_array("$.['project.monitoring.exclude']", exclude_list)
            .await?;

        let new_exclude_list: Vec<String> = exclude_list
            .iter()
            .map(|item| item.to_string_lossy().into_owned())
            .collect();

        let mut module = self.module.write().await;
        match &mut module.monitoring.exclude {
            Some(exclude) => exclude.extend(new_exclude_list.iter().cloned()),
            None => module.monitoring.exclude = Some(new_exclude_list.clone()),
        }

        Ok(new_exclude_list)
    }

    pub async fn fetch_exclude_list(&self) -> Option<Vec<String>> {
        let module_lock = self.module.read().await;

        module_lock.monitoring.exclude.clone()
    }
}

#[async_trait]
impl AsyncTryFrom<Arc<workspace::settings::Settings>> for Settings {
    type Error = anyhow::Error;

    async fn try_from_async(
        value: Arc<workspace::settings::Settings>,
    ) -> Result<Self, Self::Error> {
        let module_settings = value.get_by_key("$").await?;

        Ok(Self {
            workspace: value.clone(),
            module: Arc::new(RwLock::new(module_settings)),
        })
    }
}
