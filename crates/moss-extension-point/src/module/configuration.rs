use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value as JsonValue};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub enum ParameterValueType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
}

impl ParameterValueType {
    pub fn default_json_value(&self) -> JsonValue {
        match self {
            ParameterValueType::Number => JsonValue::Number(Number::from(0)),
            ParameterValueType::String => JsonValue::String(String::new()),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum ParameterScope {
    APPLICATION,
    #[default]
    WINDOW,
    RESOURCE,
    #[allow(non_camel_case_types)]
    LANGUAGE_SPECIFIC,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParameterValue {
    #[serde(rename = "type")]
    pub typ: ParameterValueType,
    #[serde(default)]
    pub maximum: JsonValue,
    #[serde(default)]
    pub minimum: JsonValue,
    #[serde(default)]
    pub default: JsonValue,
    /// The order in which the parameter appears within its group in the settings UI.
    pub order: Option<usize>,
    #[serde(default)]
    pub scope: ParameterScope,
    pub description: Option<String>,
    /// Excluded parameters are hidden from the UI but can still be registered.
    #[serde(default)]
    pub excluded: bool,
    /// Indicates if this setting is protected from addon overrides.
    #[serde(default)]
    pub protected: bool,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum OverrideContext {
    #[serde(rename = "*")]
    Global,
    #[serde(untagged)]
    Specific(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverrideValue {
    pub value: JsonValue,
    pub context: HashSet<OverrideContext>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationDecl {
    pub title: Option<String>,
    pub description: Option<String>,

    /// The order in which this group appears in the settings UI.
    pub order: Option<usize>,
    #[serde(rename = "parameter")]
    pub parameters: HashMap<ArcStr, Arc<ParameterValue>>,
    #[serde(rename = "override")]
    pub overrides: HashMap<ArcStr, OverrideValue>,
}
