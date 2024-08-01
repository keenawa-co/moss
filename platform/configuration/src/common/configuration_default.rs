use arc_swap::ArcSwapOption;
use std::sync::Arc;

use super::{
    configuration_model::ConfigurationModel, configuration_registry::ConfigurationRegistry, utl,
};

pub struct DefaultConfiguration {
    configuration_model: ArcSwapOption<ConfigurationModel>,
    configuration_registry: Arc<ConfigurationRegistry>,
}

impl DefaultConfiguration {
    pub fn new(registry: Arc<ConfigurationRegistry>) -> Self {
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
        let mut new_model = ConfigurationModel::empty();
        let properties = self.configuration_registry.get_configuration_properties();

        for (key, property) in properties {
            if let Some(default_value) = &property.schema.default {
                new_model.set_value(utl::format_key(key), default_value.clone());
            }
        }

        self.configuration_model.store(Some(Arc::new(new_model)))
    }
}
