use std::sync::Arc;

use anyhow::{Context as AnyhowContext, Result};
use parking_lot::Mutex;
use serde_json::Value;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{Configuration, ConfigurationLayer, Parser},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    configuration: Configuration,
    // TODO: user_configuration
    // default_configuration: DefaultConfiguration,
    registry: Arc<Mutex<ConfigurationRegistry>>,
}

impl ConfigurationService {
    pub fn new(
        registry: Arc<Mutex<ConfigurationRegistry>>,
        config_file_path: &str,
    ) -> Result<Self> {
        let parser = Parser::new();
        let workspace_configuration = parser
            .parse_file(config_file_path)
            .context(format!("failed to open file: {config_file_path}"))?;

        let conf = Configuration::new(
            DefaultConfiguration::new().0, // TODO: use default_configuration,
            ConfigurationLayer::empty(),
            workspace_configuration,
            ConfigurationLayer::empty(),
        );

        Ok(Self {
            // default_configuration,
            configuration: conf,
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
