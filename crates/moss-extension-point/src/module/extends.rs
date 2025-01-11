use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub enum ParameterType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParameterValue {
    maximum: Option<Value>,
    minimum: Option<Value>,
    #[serde(rename = "type")]
    typ: ParameterType,
    default: Value,
    description: Option<String>,
    order: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigurationDecl {
    order: Option<usize>,
    description: Option<String>,
    #[serde(rename = "parameter")]
    parameters: HashMap<String, ParameterValue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendsDecl {
    configuration: Option<ConfigurationDecl>,
}
