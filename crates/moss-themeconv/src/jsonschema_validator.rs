use serde_json::Value;
use std::sync::Arc;

use crate::Validator;

pub struct JsonSchemaValidator {
    schema: Arc<Value>,
}

impl JsonSchemaValidator {
    pub fn new(schema: Arc<Value>) -> Self {
        Self { schema }
    }
}

impl Validator for JsonSchemaValidator {
    fn validate(&self, theme_value: &Value) -> anyhow::Result<()> {
        if !jsonschema::is_valid(&self.schema, theme_value) {
            return Err(anyhow!("Schema validation failed"));
        }

        Ok(())
    }
}
