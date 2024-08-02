use anyhow::Result;

pub mod configuration_default;
pub mod configuration_model;
pub mod configuration_policy;
pub mod configuration_registry;
pub mod configuration_service;
pub mod policy;
pub mod utl;

#[async_trait]
pub trait AbstractConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>)
        -> Option<serde_json::Value>;

    async fn update_value(
        &self,
        key: &str,
        value: serde_json::Value,
        overrider_ident: Option<&str>,
    ) -> Result<()>;
}
