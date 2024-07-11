use std::{default, sync::Arc};

use anyhow::{Context as AnyhowContext, Result};
use parking_lot::Mutex;
use serde_json::Value;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{Configuration, ConfigurationModel, ConfigurationParser},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    configuration: Configuration,
    // TODO: user_configuration
    // TODO: default_configuration: DefaultConfiguration,
    registry: Arc<ConfigurationRegistry>,
}

impl ConfigurationService {
    pub fn new(registry: Arc<ConfigurationRegistry>, config_file_path: &str) -> Result<Self> {
        let parser = ConfigurationParser::new();
        let workspace_configuration = parser
            .parse_file(config_file_path)
            .context(format!("failed to open file: {config_file_path}"))?;

        let default_configuration = DefaultConfiguration::new(Arc::clone(&registry));
        default_configuration.initialize();

        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")?;

        let configuration = Configuration::new(
            default_configuration_model,
            ConfigurationModel::empty(),
            workspace_configuration,
            ConfigurationModel::empty(),
        );

        Ok(Self {
            configuration,
            registry,
        })
    }
}

impl AbstractConfigurationService for ConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value> {
        self.configuration.get_value(key, overrider_identifier)
    }

    fn update_value(&self, _key: &str, _value: &str) {
        unimplemented!()
    }
}
