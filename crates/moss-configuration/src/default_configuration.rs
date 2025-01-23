use moss_extension_point::{
    interpreter::types::configuration::ParameterType, registry::ConfigurationRegistry,
};
use serde_json::Value as JsonValue;
use std::sync::Arc;

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
            let default_value = if let Some(override_descriptor) = registry.get_override(key) {
                override_descriptor.value.clone()
            } else if !value.default.is_null() {
                value.default.clone()
            } else {
                type_default_json_value(&value.typ)
            };

            if !model.insert(key, default_value) {
                warn!("Parameter '{key}' already exists in the default configuration model");
                continue;
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

pub fn type_default_json_value(typ: &ParameterType) -> JsonValue {
    match typ {
        ParameterType::Number => 0.into(),
        ParameterType::String => "".into(),
        ParameterType::Bool => false.into(),
    }
}
