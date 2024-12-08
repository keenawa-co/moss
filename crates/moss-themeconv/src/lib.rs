pub mod json_converter;
pub mod jsonschema_validator;

mod util;

use anyhow::Result;
use serde_json::Value;

#[macro_use]
extern crate anyhow;

pub trait ThemeConverter {
    fn convert_to_css(&self, content: String) -> Result<String>;
}

pub trait Validator {
    fn validate(&self, theme_value: &Value) -> Result<()>;
}
