use anyhow::{Context as AnyhowContext, Result};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{
        Configuration, ConfigurationModel, ConfigurationParser, UserConfiguration,
    },
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    default_configuration: DefaultConfiguration,
    user_configuration: UserConfiguration,
    configuration: Configuration,
}

impl ConfigurationService {
    pub fn new(registry: Arc<ConfigurationRegistry>, config_file_path: &PathBuf) -> Result<Self> {
        let parser = ConfigurationParser::new(Arc::clone(&registry));
        let user_configuration = UserConfiguration::new(config_file_path, Arc::new(parser));

        let default_configuration = DefaultConfiguration::new(Arc::clone(&registry));
        default_configuration.initialize();

        let user_configuration_model = user_configuration
            .load_configuration()
            .context("failed to load user configuration model")?;
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")?;

        let configuration = Configuration::new(
            default_configuration_model,
            user_configuration_model,
            ConfigurationModel::empty(),
            ConfigurationModel::empty(),
        );

        Ok(Self {
            default_configuration,
            user_configuration,
            configuration,
        })
    }
}

impl AbstractConfigurationService for ConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value> {
        self.configuration.get_value(key, overrider_identifier)
    }

    // TODO: use type Keyable for key
    fn update_value(&self, key: &str, _value: serde_json::Value) {}
}

#[derive(Debug)]
pub struct ConfigurationEditingService {
    edited_resource: PathBuf,
    write_queue: UnboundedSender<ConfigurationWriteJob>,
}

#[derive(Debug)]
pub struct ConfigurationWriteJob {
    path: String, // JSON Path
    value: serde_json::Value,
}

impl ConfigurationEditingService {
    fn new(edited_resource: PathBuf) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
            edited_resource,
            write_queue: tx,
        }
    }

    fn write(&self, path: String, value: serde_json::Value) -> Result<()> {
        unimplemented!()
    }

    fn enqueue_write_job(&self, job: ConfigurationWriteJob) -> Result<()> {
        Ok(self.write_queue.send(job)?)
    }

    fn do_write_job(&self, job: ConfigurationWriteJob) {
        unimplemented!()
    }
}
