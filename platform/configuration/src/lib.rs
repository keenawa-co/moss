pub mod configuration_default;
pub mod configuration_model;
pub mod configuration_parser;
pub mod configuration_policy;
pub mod configuration_registry;
pub mod configuration_service;
pub mod policy;
pub mod user_settings;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate lazy_regex;

use anyhow::Result;
use configuration_model::AttributeName;

#[async_trait]
pub trait AbstractConfigurationService {
    fn get_value(&self, attribute_name: AttributeName) -> Option<serde_json::Value>;

    async fn update_value(
        &self,
        attribute_name: AttributeName,
        value: &serde_json::Value,
    ) -> Result<()>;
}
