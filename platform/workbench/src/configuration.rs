use serde_json::Value;

pub mod configuration_default;
pub mod configuration_model;
pub mod configuration_registry;
pub mod configuration_service;

pub trait AbstractConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value>;
    fn update_value(&self, key: &str, value: &str);
}
