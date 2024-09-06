use arc_swap::ArcSwapOption;
use platform_core::common::context::{entity::Model, Context};
use radix_trie::Trie;
use std::sync::Arc;

use super::{
    configuration_model::{AttributeName, ConfigurationModel},
    configuration_registry::ConfigurationRegistry,
};

pub struct DefaultConfiguration {
    configuration_model: ArcSwapOption<ConfigurationModel>,
    configuration_registry: Model<ConfigurationRegistry>,
}

impl DefaultConfiguration {
    pub fn new(registry: Model<ConfigurationRegistry>) -> Self {
        Self {
            configuration_model: ArcSwapOption::from(None),
            configuration_registry: registry,
        }
    }

    pub fn initialize(&self, ctx: &mut Context) {
        self.reset_configuration_model(ctx);
    }

    pub fn get_configuration_model(&self) -> Option<Arc<ConfigurationModel>> {
        self.configuration_model.load_full()
    }

    fn reset_configuration_model(&self, ctx: &mut Context) {
        let configuration_registry = self.configuration_registry.read(ctx);
        let properties = configuration_registry.properties();
        let mut new_model = ConfigurationModel {
            content: Trie::new(),
            names: Vec::new(),
            overrides: configuration_registry
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
