use configuration::common::configuration_model::{
    AttributeName, Configuration as PlatformConfiguration,
    ConfigurationParser as PlatformConfigurationParser,
};
use serde_json::Value;

pub struct WorkspaceConfigurationParser {
    platform_parser: PlatformConfigurationParser,
}

pub struct WorkspaceConfiguration {
    platform_configuration: PlatformConfiguration,
}

impl WorkspaceConfiguration {
    pub fn new(platform_configuration: PlatformConfiguration) -> Self {
        Self {
            platform_configuration,
        }
    }

    pub fn get_value(&self, attribute_name: &AttributeName) -> Option<Value> {
        self.platform_configuration.get_value(attribute_name)
    }
}
