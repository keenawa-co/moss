use std::sync::Arc;

use moss_extension_point::registry::ConfigurationRegistry;

use crate::ConfigurationModel;

pub struct DefaultConfiguration {
    // OPTIMIZE: It probably makes sense to use `ArcSwap` here,
    // as we might theoretically need the ability to completely replace
    // the default settings model during a reset.
    //
    // Revisit this question when the use cases and functionality become clearer.
    model: Arc<ConfigurationModel>,
}

impl DefaultConfiguration {
    pub fn new(registry: Arc<ConfigurationRegistry>) -> Self {
        let mut model = ConfigurationModel::new();
        for (key, value) in registry.parameters() {
            // TODO: Check if there are overrides for the given key

            let default_value = if value.default.is_null() {
                value.typ.default_json_value()
            } else {
                value.default.clone()
            };

            if !model.insert(key, default_value) {
                warn!("Parameter '{key}' already exists in the default configuration model")
            }
        }

        Self {
            model: Arc::new(model),
        }
    }

    pub fn model(&self) -> &Arc<ConfigurationModel> {
        &self.model
    }
}
