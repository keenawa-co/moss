pub mod default_configuration;

use arcstr::ArcStr;
use serde_json::Value as JsonValue;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[macro_use]
extern crate tracing;

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

pub struct ConfigurationModel {
    /// A JSON object with string keys, where the values are specific settings.
    pub contents: HashMap<ArcStr, JsonValue>,
    /// A set of all keys present in this object.
    pub keys: HashSet<ArcStr>,
    /// A list of override blocks
    pub overrides: HashMap<ArcStr, OverrideObject>,
}

impl ConfigurationModel {
    pub fn new() -> Self {
        Self {
            contents: HashMap::new(),
            keys: HashSet::new(),
            overrides: HashMap::new(),
        }
    }

    pub fn get(&self, key: &ArcStr) -> Option<&JsonValue> {
        self.contents.get(key)
    }

    pub fn insert(&mut self, key: &ArcStr, value: JsonValue) -> bool {
        if !self.keys.insert(ArcStr::clone(&key)) {
            return false;
        }

        self.contents.insert(ArcStr::clone(&key), value);
        return true;
    }
}

pub struct Configuration {
    default: Arc<ConfigurationModel>,
    // TODO:
    // user: ConfigurationModel,
    // workspace: ConfigurationModel,
    // inmem: ConfigurationModel,
    // consolidated: ConfigurationModel,
}

impl Configuration {
    pub fn new(default: Arc<ConfigurationModel>) -> Self {
        Self { default }
    }

    pub fn get_value(&self, key: &str) -> Option<&JsonValue> {
        self.default.get(&ArcStr::from(key))
    }
}
