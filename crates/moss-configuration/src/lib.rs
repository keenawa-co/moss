use std::collections::{HashMap, HashSet};

use serde_json::Value as JsonValue;

#[derive(Debug)]
pub enum Target {}

pub struct OverrideObject {
    /// A set of all keys that are overridden in this object.
    keys: HashSet<String>,
    /// An object that stores the actual data for these overridden keys.
    value: JsonValue,
    /// A set of identifiers that own this override.
    identifiers: HashSet<String>,
}

pub struct ConfigurationObject {
    /// A JSON object with string keys, where the values are specific settings.
    value: JsonValue,
    /// A set of all keys present in this object.
    keys: HashSet<String>,
    /// A list of override blocks
    overrides: HashMap<String, OverrideObject>,
}

pub struct Configuration {
    default: ConfigurationObject,
    user: ConfigurationObject,
    workspace: ConfigurationObject,
    inmem: ConfigurationObject,
    consolidated: ConfigurationObject,
}
