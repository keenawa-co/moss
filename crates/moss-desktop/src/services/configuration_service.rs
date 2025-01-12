use anyhow::{Context as _, Result};
use moss_configuration::{default_configuration::DefaultConfiguration, Configuration};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use std::sync::Arc;

use crate::app::service::Service;

pub struct ConfigurationService {
    default_configurations: DefaultConfiguration,
    configuration: Configuration,
}

impl ConfigurationService {
    pub fn new(default_configurations: DefaultConfiguration) -> Self {
        let configuration = Configuration::new(Arc::clone(default_configurations.model()));

        Self {
            default_configurations,
            configuration,
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&JsonValue> {
        self.configuration.get_value(key)
    }

    pub fn get_typed_value<T>(&self, key: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let value = self
            .get_value(key)
            .with_context(|| format!("Key '{}' not found in configuration", key))?;

        serde_json::from_value(value.clone())
            .with_context(|| format!("Failed to deserialize key '{}' into target type", key))
    }
}

impl Service for ConfigurationService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
