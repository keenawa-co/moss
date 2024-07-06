use std::sync::Arc;

use parking_lot::Mutex;
use serde_json::Value;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{ConfigurationModel, ConfigurationModelParser},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    configuration: ConfigurationModel,
    default_configuration: DefaultConfiguration,
    registry: Arc<Mutex<ConfigurationRegistry>>,
}

impl ConfigurationService {
    pub fn new(registry: Arc<Mutex<ConfigurationRegistry>>, config_file_path: &str) -> Self {
        let parser = ConfigurationModelParser::new();
        parser.parse_file(config_file_path).unwrap();
        todo!()
        // Self {
        //     default_configuration: DefaultConfiguration::new(),
        //     user_configuration: parser
        //         .parse_file(config_file_path)
        //         .unwrap_or_else(|_| ConfigurationModel::new()),
        //     configuration: ConfigurationModel::new(),
        //     registry,
        // }
    }
}

// impl AbstractConfigurationService for ConfigurationService {
//     fn get_value(&self, section: Option<&str>) -> Option<&Value> {
//         self.configuration.get_value(section)
//     }

//     fn update_value(&self, _key: &str, _value: &str) {
//         unimplemented!()
//     }
// }
