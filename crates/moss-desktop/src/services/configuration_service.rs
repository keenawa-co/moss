use std::sync::LazyLock;

use serde_json::{json, Value as JsonValue};

use crate::app::service::Service;

// contribution::submit!(Registry);
// contribution::submit!(ConfigurationRegistry, {

// });

pub struct ConfigurationRegistry {}

pub struct Registry {}

impl Registry {
    pub fn as_<T>(&self) -> T {
        todo!()
    }
}

static REGISTRY: LazyLock<Registry> = LazyLock::new(|| Registry {});

static CONTRIB: LazyLock<()> = LazyLock::new(|| {
    let value: ConfigurationRegistry = REGISTRY.as_::<ConfigurationRegistry>();
});

pub struct ConfigurationService {}

// pub struct JsonSchema {
//     properties:
// }

impl ConfigurationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_configuration(&self) {
        let schema = json!(
            {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "$id": "Configuration.json",
                "type": "object",
                "properties": {
                    "window.defaultWidth": {
                        "type": "integer",
                        "default": 800,
                        "minimum": 800,
                        "maximum": 3840,
                        "contentMediaType": "APPLICATION",
                        "description": "The width of the application window in pixels."
                    },
                    "window.defaultHeight": {
                        "type": "integer",
                        "default": 600,
                        "minimum": 600,
                        "maximum": 2160,
                        "contentMediaType": "APPLICATION",
                        "description": "The height of the application window in pixels.",
                        "title": "Application"
                    }
                }
            }
        );
    }

    fn parse_schema(&self, schema: &JsonValue) {}
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
