use anyhow::Result;
use arc_swap::{ArcSwap, ArcSwapOption};
use hashbrown::{HashMap, HashSet};
use lazy_regex::{Lazy, Regex};
use moss_base::collection::Extend;
use radix_trie::{Trie, TrieCommon};
use serde_json::Value;
use std::{fs::File, io::Read, path::PathBuf, sync::Arc, vec};

use super::{configuration_registry::ConfigurationRegistry, utl};

// TODO:
// - Use a kernel/fs to work with the file system
// - Use a LogService.
// - Use a PolicyService
pub struct UserConfiguration {
    content_parser: Arc<ConfigurationParser>,
    configuration_resource: PathBuf,
}

impl UserConfiguration {
    pub fn new(file_path: &PathBuf, content_parser: Arc<ConfigurationParser>) -> Self {
        Self {
            content_parser,
            configuration_resource: file_path.clone(),
        }
    }

    pub fn load_configuration(&self) -> Result<ConfigurationModel> {
        let mut file = File::open(&self.configuration_resource)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if content.trim().is_empty() {
            content = String::from("{}")
        }

        Ok(self.content_parser.parse(&content)?)
    }
}

/// Enum representing the various configuration targets in Moss Compass.
/// These targets specify where the configuration settings should be applied.
#[derive(Debug)]
pub enum ConfigurationTarget {
    /// Platform specific configuration.
    /// These settings apply to the entire application and cannot be overridden in local, workspace, etc.
    Platform,

    /// User specific configuration.
    /// These settings apply globally to the user and can be overridden in workspace or others settings.
    User,

    /// Workspace specific configuration.
    /// These settings apply to the specific workspace and can override by`ConfigurationTarget::User` settings.
    Workspace,

    /// Default configuration.
    /// These settings represent the default values provided by Moss Compass and can be overridden by any other configuration target.
    Default,

    /// Memory specific configuration.
    /// These settings are stored in memory and are not persisted. They can be used for temporary configurations.
    Memory,
}

#[derive(Debug)]
pub struct ConfigurationModel {
    content: Trie<String, Value>,
    keys: Vec<String>,
    content_overrides: Trie<String, Value>,
    override_identifiers: Vec<String>,
}

impl Clone for ConfigurationModel {
    fn clone(&self) -> Self {
        ConfigurationModel {
            content: self.content.clone(),
            keys: self.keys.clone(),
            content_overrides: self.content_overrides.clone(),
            override_identifiers: self.override_identifiers.clone(),
        }
    }
}

impl ConfigurationModel {
    pub fn new(
        content: Trie<String, Value>,
        keys: Vec<String>,
        content_overrides: Trie<String, Value>,
        override_identifiers: Vec<String>,
    ) -> Self {
        Self {
            content,
            keys,
            content_overrides,
            override_identifiers,
        }
    }

    pub fn empty() -> Self {
        Self {
            content: Trie::new(),
            keys: Vec::new(),
            content_overrides: Trie::new(),
            override_identifiers: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty() && self.keys.len() == 0 && self.override_identifiers.len() == 0
    }

    pub fn get_keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub fn get_value(&self, key: &str, override_ident: Option<&str>) -> Option<&Value> {
        if let Some(ident) = override_ident {
            self.content_overrides
                .get(&format!("$.[{}].{}", ident, key))
        } else {
            self.content.get(&format!("$.{}", key))
        }
    }

    pub fn set_value(&mut self, key: String, value: serde_json::Value) {
        self.content.insert(key.clone(), value);
        self.keys.push(key);
    }

    // TODO:
    // fn get_override_identifier_content(&self, identifier: &str) -> Option<HashMap<String, Value>> {
    //     self.overrides
    //         .iter()
    //         .find(|override_data| override_data.identifier == identifier)
    //         .map(|override_data| override_data.content.clone())
    // }

    pub fn merge(&self, others: &[Arc<ConfigurationModel>]) -> Self {
        let mut result = self.clone();

        for other in others {
            result.content.extend(other.content.iter());

            let new_keys: Vec<String> = other
                .keys
                .iter()
                .map(|k| dbg!(k))
                .filter(|key| !result.keys.contains(key))
                .cloned()
                .collect();

            result.keys.extend(new_keys);
            result
                .content_overrides
                .extend(other.content_overrides.clone().iter());
            result
                .override_identifiers
                .extend(other.override_identifiers.clone());
        }

        result
    }
}

static OVERRIDE_PROPERTY_REGEX: &'static Lazy<Regex> = regex!(r"^(\[.*\])+$");

pub struct ConfigurationParser {
    registry: Arc<ConfigurationRegistry>,
}

struct ConfigurationOverride {
    ident: String,
    keys: Vec<String>,
    content: Trie<String, serde_json::Value>,
}

impl ConfigurationParser {
    pub fn new(registry: Arc<ConfigurationRegistry>) -> Self {
        Self { registry }
    }

    pub fn parse(&self, content: &str) -> Result<ConfigurationModel> {
        let raw_content: HashMap<String, Value> = serde_json::from_str(content)?;
        let mut model = ConfigurationModel::empty();

        let configuration_properties = self.registry.get_configuration_properties();

        for (key, value) in &raw_content {
            if OVERRIDE_PROPERTY_REGEX.is_match(key) {
                if let Some(override_definition) = self.process_override_definition(key, value) {
                    model.override_identifiers.push(override_definition.ident);
                    model
                        .content_overrides
                        .extend(override_definition.content.iter());
                }

                continue;
            }

            match configuration_properties.get(key) {
                Some(registered_property) => {
                    if registered_property.is_protected() {
                        println!("Property `{}` is protected from contribution", key);
                        continue;
                    }

                    let formatted_key = utl::format_key(key);

                    model.content.insert(formatted_key.clone(), value.clone());
                    model.keys.push(formatted_key);
                }
                None => {
                    println!("Unknown property `{}` was detected", key);
                    continue;
                }
            }
        }

        Ok(model)
    }

    fn process_override_definition(
        &self,
        key: &str,
        value: &Value,
    ) -> Option<ConfigurationOverride> {
        let content = if let Value::Object(obj) = value {
            obj
        } else {
            // If the override is not an object, then we don't want to handle it in any way.
            return None;
        };

        let override_identifiers = self.registry.get_override_identifiers();
        let formatted_identifier = key.trim_matches(|c| c == '[' || c == ']');

        if override_identifiers.get(formatted_identifier).is_none() {
            println!(
                "Unknown override identifier `{}` was detected",
                formatted_identifier
            );
            return None;
        }

        let mut result = ConfigurationOverride {
            ident: formatted_identifier.to_string(),
            keys: Vec::new(),
            content: Trie::new(),
        };

        let configuration_properties = self.registry.get_configuration_properties();
        for (key, value) in content {
            match configuration_properties.get(key) {
                Some(registered_property) => {
                    if registered_property.is_protected() {
                        println!("Property `{}` is protected from contribution", key);
                        continue;
                    }

                    let formatted_key = format!("$.[{}].{}", formatted_identifier, key);

                    result.content.insert(formatted_key.clone(), value.clone());
                    result.keys.push(formatted_key);
                }
                None => {
                    println!("Unknown property `{}` was detected", key);
                    continue;
                }
            }
        }

        Some(result)
    }
}

#[derive(Debug)]
pub struct InspectedConfigurationValue {
    key: String,
    value: Option<serde_json::Value>,
    seen_in_overrides: Vec<String>,
    default_configuration: Arc<ConfigurationModel>,
    policy_configuration: Arc<ConfigurationModel>,
}

impl InspectedConfigurationValue {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &Option<serde_json::Value> {
        &self.value
    }
    // TODO: Rewrite using override keys and getting sections
    pub fn get_default_value(
        &self,
        key: &str,
        overrider_ident: Option<&str>,
    ) -> Option<&serde_json::Value> {
        self.default_configuration.get_value(key, overrider_ident)
    }

    // TODO: Rewrite using override keys and getting sections
    pub fn get_policy_value(
        &self,
        key: &str,
        overrider_ident: Option<&str>,
    ) -> Option<&serde_json::Value> {
        self.policy_configuration.get_value(key, overrider_ident)
    }
}

#[derive(Debug)]
pub struct Configuration {
    default_configuration: Arc<ConfigurationModel>,
    policy_configuration: Arc<ConfigurationModel>,
    user_configuration: ArcSwap<ConfigurationModel>,
    workspace_configuration: Arc<ConfigurationModel>,
    inmem_configuration: Arc<ConfigurationModel>,
    consolidated_configuration: ArcSwapOption<ConfigurationModel>,
}

// TODO: add overrides
pub struct ConfigurationDifference {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

impl Configuration {
    pub fn new(
        default_model: Arc<ConfigurationModel>,
        policy_model: Arc<ConfigurationModel>,
        user_model: ConfigurationModel,
        workspace_model: ConfigurationModel,
        inmem_model: ConfigurationModel,
    ) -> Self {
        Configuration {
            default_configuration: default_model,
            policy_configuration: policy_model,
            user_configuration: ArcSwap::new(Arc::new(user_model)),
            workspace_configuration: Arc::new(workspace_model),
            inmem_configuration: Arc::new(inmem_model),
            consolidated_configuration: ArcSwapOption::from(None),
        }
    }

    // TODO: implement `section` functionality

    pub fn get_value(&self, key: &str, overrider_ident: Option<&str>) -> Option<Value> {
        let consolidated_model = self.get_consolidated_configuration();

        consolidated_model.get_value(key, overrider_ident).cloned()
    }

    pub fn inspect(&self, key: &str, overrider_ident: Option<&str>) -> InspectedConfigurationValue {
        let consolidated_model = self.get_consolidated_configuration();

        let value = consolidated_model.get_value(key, overrider_ident).cloned();

        let mut inspected_value = InspectedConfigurationValue {
            key: key.to_string(),
            value,
            seen_in_overrides: Vec::new(),
            default_configuration: Arc::clone(&self.default_configuration),
            policy_configuration: Arc::clone(&self.policy_configuration),
        };

        for ident in consolidated_model.override_identifiers.iter() {
            if consolidated_model
                .content_overrides
                .get(&format!("$.[{}].{}", ident, key))
                .is_some()
            {
                inspected_value.seen_in_overrides.push(ident.to_string());
            }
        }

        inspected_value
    }

    pub fn update_user_configuration(
        &self,
        new_model: Arc<ConfigurationModel>,
    ) -> ConfigurationDifference {
        let diff = Self::compare(self.user_configuration.load_full(), Arc::clone(&new_model));
        self.user_configuration.swap(new_model);
        self.consolidated_configuration.swap(None);

        diff
    }

    fn compare(
        old: Arc<ConfigurationModel>,
        new: Arc<ConfigurationModel>,
    ) -> ConfigurationDifference {
        let old_keys: HashSet<_> = old.keys.iter().cloned().collect();
        let new_keys: HashSet<_> = new.keys.iter().cloned().collect();

        ConfigurationDifference {
            added: new_keys.difference(&old_keys).cloned().collect(),
            removed: old_keys.difference(&new_keys).cloned().collect(),
            modified: old_keys
                .intersection(&new_keys)
                .filter(
                    |key| match (old.get_value(key, None), new.get_value(key, None)) {
                        (Some(old_value), Some(new_value)) => old_value != new_value,
                        _ => false,
                    },
                )
                .cloned()
                .collect(),
        }
    }

    pub fn get_consolidated_configuration(&self) -> Arc<ConfigurationModel> {
        if let Some(config) = self.consolidated_configuration.load_full().as_ref() {
            return Arc::clone(config);
        }

        dbg!(&self.default_configuration);

        let consolidated_model = self
            .default_configuration
            .merge(&[
                Arc::clone(&self.user_configuration.load_full()),
                Arc::clone(&self.workspace_configuration),
                Arc::clone(&self.inmem_configuration),
            ])
            .into();

        self.consolidated_configuration
            .store(Some(Arc::clone(&consolidated_model)));

        consolidated_model
    }
}
