use std::collections::{HashMap, HashSet};

use serde_json::Value as JsonValue;

#[repr(u8)]
#[derive(Debug)]
pub enum Level {
    Application = 1,
    Machine,
    Window,
    Resource,
    LanguageSpecific,
    MachineOverridable,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Target {
    Application = 1,
    User,
    Workspace,
}

pub struct OverrideObject {
    /// A set of all keys that are overridden in this object.
    pub keys: HashSet<String>,
    /// An object that stores the actual data for these overridden keys.
    pub value: JsonValue,
    /// A set of identifiers that own this override.
    pub identifiers: HashSet<String>,
}

pub struct ConfigurationObject {
    /// A JSON object with string keys, where the values are specific settings.
    pub value: JsonValue,
    /// A set of all keys present in this object.
    pub keys: HashSet<String>,
    /// A list of override blocks
    pub overrides: HashMap<String, OverrideObject>,
}

pub struct Configuration {
    default: ConfigurationObject,
    user: ConfigurationObject,
    workspace: ConfigurationObject,
    inmem: ConfigurationObject,
    consolidated: ConfigurationObject,
}
