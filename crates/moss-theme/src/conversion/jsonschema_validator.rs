use anyhow::anyhow;
use serde_json::Value;

use super::Validator;

pub struct JsonSchemaValidator {
    schema: &'static Value,
}

impl JsonSchemaValidator {
    pub fn new(schema: &'static Value) -> Self {
        Self { schema }
    }
}

impl Validator for JsonSchemaValidator {
    fn validate(&self, theme_value: &Value) -> anyhow::Result<()> {
        if !jsonschema::is_valid(self.schema, theme_value) {
            return Err(anyhow!("Schema validation failed"));
        }

        Ok(())
    }
}
