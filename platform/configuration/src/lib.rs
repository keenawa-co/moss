pub mod configuration_default;
pub mod configuration_model;
pub mod configuration_registry;
pub mod configuration_service;

use serde_json::Value;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_regex;

pub trait AbstractConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value>;
    fn update_value(&self, key: &str, value: &str);
}
