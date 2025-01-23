use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use hcl::{
    eval::{Context, Evaluate},
    Expression, Map, Value as HclValue,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::str::FromStr;
use std::sync::Arc;
use strum::EnumString as StrumEnumString;

#[derive(Debug, Deserialize)]
pub enum ParameterType {
    Number,
    String,
    Bool,
}

impl TryFrom<hcl::Variable> for ParameterType {
    type Error = anyhow::Error;

    fn try_from(value: hcl::Variable) -> std::result::Result<Self, Self::Error> {
        match value.to_string().as_str() {
            "number" => Ok(ParameterType::Number),
            "string" => Ok(ParameterType::String),
            "bool" => Ok(ParameterType::Bool),
            _ => Err(anyhow!("unknown type")),
        }
    }
}

#[derive(Debug, Default, StrumEnumString)]
pub enum ParameterScope {
    APPLICATION,
    #[default]
    WINDOW,
    RESOURCE,
    #[allow(non_camel_case_types)]
    LANGUAGE_SPECIFIC,
}

#[derive(Debug)]
pub struct ConfigurationNode {
    pub ident: ArcStr,
    pub parent_ident: Option<ArcStr>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub order: Option<u64>,
    pub parameters: HashMap<ArcStr, Arc<Parameter>>,
    pub overrides: HashMap<ArcStr, Arc<Override>>,
}

#[derive(Debug)]
pub struct Parameter {
    pub ident: ArcStr,
    pub typ: ParameterType,
    pub maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub default: JsonValue,
    /// The order in which the parameter appears within its group in the settings UI.
    pub order: Option<u64>,
    pub scope: ParameterScope,
    pub description: Option<String>,
    /// Excluded parameters are hidden from the UI but can still be registered.
    pub excluded: bool,
    /// Indicates if this setting is protected from addon overrides.
    pub protected: bool,
}

#[derive(Debug)]
pub struct Override {
    pub ident: ArcStr,
    pub value: JsonValue,
    pub context: Option<HashSet<String>>,
}
