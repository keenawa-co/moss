use std::{default, sync::Arc};

use parking_lot::Mutex;
use serde_json::Value;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{ConfigurationEntryModel, ConfigurationModel, ConfigurationModelParser},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    configuration: ConfigurationModel,
    // TODO: user_configuration
    // default_configuration: DefaultConfiguration,
    registry: Arc<Mutex<ConfigurationRegistry>>,
}

impl ConfigurationService {
    pub fn new(registry: Arc<Mutex<ConfigurationRegistry>>, config_file_path: &str) -> Self {
        let parser = ConfigurationModelParser::new();
        let workspace_configuration = parser.parse_file(config_file_path).unwrap();
        let default_configuration = DefaultConfiguration::new();
        let conf = ConfigurationModel::new(
            DefaultConfiguration::new().0, // TODO: use default_configuration,
            ConfigurationEntryModel::empty(),
            workspace_configuration,
            ConfigurationEntryModel::empty(),
        );

        Self {
            // default_configuration,
            configuration: conf,
            registry,
        }
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
