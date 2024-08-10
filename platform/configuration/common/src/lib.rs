pub mod platform_configuration_default;
pub mod platform_configuration_model;
pub mod platform_configuration_policy;
pub mod platform_configuration_registry;
pub mod platform_configuration_service;
pub mod policy;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate lazy_regex;

use anyhow::Result;
use platform_configuration_model::AttributeName;

#[async_trait]
pub trait AbstractConfigurationService {
    fn get_value(&self, attribute_name: AttributeName) -> Option<serde_json::Value>;

    async fn update_value(
        &self,
        attribute_name: AttributeName,
        value: &serde_json::Value,
    ) -> Result<()>;
}
