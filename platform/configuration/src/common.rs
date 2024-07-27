use anyhow::Result;
use configuration_registry::Keyable;

pub mod configuration_default;
pub mod configuration_model;
pub mod configuration_policy;
pub mod configuration_registry;
pub mod configuration_service;
pub mod policy;

pub trait AbstractConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>)
        -> Option<serde_json::Value>;
    fn update_value(&self, key: impl Keyable, value: serde_json::Value) -> Result<()>;
}
