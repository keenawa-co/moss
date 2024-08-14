use arc_swap::ArcSwapOption;
use radix_trie::Trie;
use std::sync::Arc;

use super::{
    configuration_model::{AttributeName, ConfigurationModel},
    configuration_registry::ConfigurationRegistry,
};

pub struct DefaultConfiguration<'a> {
    configuration_model: ArcSwapOption<ConfigurationModel>,
    configuration_registry: &'a ConfigurationRegistry<'a>,
}

impl<'a> DefaultConfiguration<'a> {
    pub fn new(registry: &'a ConfigurationRegistry<'a>) -> Self {
        Self {
            configuration_model: ArcSwapOption::from(None),
            configuration_registry: registry,
        }
    }

    pub fn initialize(&self) {
        self.reset_configuration_model();
    }

    pub fn get_configuration_model(&self) -> Option<Arc<ConfigurationModel>> {
        self.configuration_model.load_full()
    }

    fn reset_configuration_model(&self) {
        let properties = self.configuration_registry.properties();
        let mut new_model = ConfigurationModel {
            content: Trie::new(),
            names: Vec::new(),
            overrides: self
                .configuration_registry
                .override_identifiers()
                .iter()
                .cloned()
                .collect(),
        };

        for (key, property) in properties {
            if let Some(default_value) = &property.schema.default {
                new_model.set_value(AttributeName::format(key), default_value.clone());
            }
        }

        self.configuration_model.store(Some(Arc::new(new_model)))
    }
}
