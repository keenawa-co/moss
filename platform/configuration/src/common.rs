use anyhow::Result;
use configuration_registry::Keyable;

pub mod configuration_default;
pub mod configuration_model;
pub mod configuration_policy;
pub mod configuration_registry;
pub mod configuration_service;
pub mod policy;

#[async_trait]
pub trait AbstractConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>)
        -> Option<serde_json::Value>;
    async fn update_value(&self, key: impl Keyable + Send, value: serde_json::Value) -> Result<()>;
}
