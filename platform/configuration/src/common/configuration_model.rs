use anyhow::Result;
use arc_swap::{ArcSwap, ArcSwapOption};
use hashbrown::HashMap;
use lazy_regex::{Lazy, Regex};
use serde_json::Value;
use std::{fs::File, io::Read, path::PathBuf, sync::Arc, vec};

use super::configuration_registry::ConfigurationRegistry;

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

#[derive(Debug, Clone)]
pub struct ConfigurationOverride {
    identifier: String,
    _keys: Vec<String>,
    contents: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationModel {
    content: HashMap<String, Value>,
    keys: Vec<String>,
    overrides: Vec<ConfigurationOverride>,
    overridden_configurations: Arc<ArcSwap<HashMap<String, Arc<ConfigurationModel>>>>,
}

impl ConfigurationModel {
    pub fn new(
        contents: HashMap<String, Value>,
        keys: Vec<String>,
        overrides: Vec<ConfigurationOverride>,
    ) -> Self {
        Self {
            content: contents,
            keys,
            overrides,
            overridden_configurations: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
        }
    }

    pub fn empty() -> Self {
        Self {
            content: HashMap::new(),
            keys: Vec::new(),
            overrides: Vec::new(),
            overridden_configurations: Arc::new(ArcSwap::new(Arc::new(HashMap::new()))),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.content.get(key)
    }

    pub fn set_value(&mut self, key: String, value: serde_json::Value) {
        self.content.insert(key.clone(), value);
        self.keys.push(key);
    }

    fn r#override(&self, identifier: &str) -> Arc<Self> {
        let current_overrides = self.overridden_configurations.load_full();

        if let Some(override_model) = current_overrides.get(identifier) {
            return Arc::clone(override_model);
        }

        let new_override = Arc::new(self.create_overridden_configuration(identifier));

        let mut new_overrides = HashMap::clone(&*current_overrides);
        new_overrides.insert(identifier.to_string(), Arc::clone(&new_override));

        self.overridden_configurations
            .store(Arc::new(new_overrides));

        new_override
    }

    fn create_overridden_configuration(&self, identifier: &str) -> Self {
        if let Some(override_content) = self.get_override_identifier_content(identifier) {
            let mut content = self.content.clone();
            content.extend(override_content);

            ConfigurationModel::new(content, self.keys.clone(), self.overrides.clone())
        } else {
            self.clone()
        }
    }

    fn get_override_identifier_content(&self, identifier: &str) -> Option<HashMap<String, Value>> {
        self.overrides
            .iter()
            .find(|override_data| override_data.identifier == identifier)
            .map(|override_data| override_data.contents.clone())
    }

    pub fn merge(&self, others: &[Arc<ConfigurationModel>]) -> Self {
        let mut merged_content = self.content.clone();
        let mut merged_keys = self.keys.clone();
        let mut merged_overrides = self.overrides.clone();

        for other in others {
            merged_content.extend(other.content.clone());

            let new_keys: Vec<String> = other
                .keys
                .iter()
                .filter(|key| !merged_keys.contains(key))
                .cloned()
                .collect();

            merged_keys.extend(new_keys);
            merged_overrides.extend(other.overrides.clone());
        }

        ConfigurationModel::new(merged_content, merged_keys, merged_overrides)
    }
}

static OVERRIDE_PROPERTY_REGEX: &'static Lazy<Regex> = regex!(r"^(\[.*\])+$");

pub struct ConfigurationParser {
    registry: Arc<ConfigurationRegistry>,
}

pub struct ConfigurationParserSettings {}

impl ConfigurationParser {
    pub fn new(registry: Arc<ConfigurationRegistry>) -> Self {
        Self { registry }
    }

    pub fn parse(&self, content: &str) -> Result<ConfigurationModel> {
        let root_map: HashMap<String, Value> = serde_json::from_str(content)?;
        let mut root_overrides: Vec<ConfigurationOverride> = Vec::new();
        let mut root_contents: HashMap<String, Value> = HashMap::new();
        let mut root_keys: Vec<String> = Vec::new();

        let configuration_properties = self.registry.get_configuration_properties();

        for (key, value) in &root_map {
            if OVERRIDE_PROPERTY_REGEX.is_match(key) {
                root_overrides.extend(self.handle_override(key, value, None));
                continue;
            }

            match configuration_properties.get(key) {
                Some(registered_property) => {
                    if registered_property.is_protected() {
                        println!("Property `{}` is protected from contribution", key);
                        continue;
                    }

                    root_contents.insert(key.clone(), value.clone());
                    root_keys.push(key.clone());
                }
                None => {
                    println!("Unknown property `{}` was detected", key);
                    continue;
                }
            }
        }

        let result = ConfigurationModel::new(root_contents, root_keys, root_overrides);

        Ok(result)
    }

    fn handle_override(
        &self,
        key: &str,
        value: &Value,
        parent_identifier: Option<&str>,
    ) -> Vec<ConfigurationOverride> {
        let content = if let Value::Object(obj) = value {
            obj
        } else {
            // If the override is not an object, then we don't want to handle it in any way.
            return vec![];
        };

        let override_identifiers = self.registry.get_override_identifiers();
        let formatted_identifier = {
            let trimmed_key = key.trim_matches(|c| c == '[' || c == ']');

            if let Some(parent_id) = parent_identifier {
                format!("{}/{}", parent_id, trimmed_key)
            } else {
                trimmed_key.to_string()
            }
        };

        if override_identifiers.get(&formatted_identifier).is_none() {
            println!(
                "Unknown override identifier `{}` was detected",
                formatted_identifier
            );
            return vec![];
        }

        let (override_overrides, parsed_override_content, override_keys) =
            self.extract_override_content_and_keys(Some(&formatted_identifier), content);

        let mut result = vec![ConfigurationOverride {
            identifier: formatted_identifier.to_string(),
            _keys: override_keys,
            contents: parsed_override_content,
        }];
        result.extend(override_overrides);

        result
    }

    fn extract_override_content_and_keys(
        &self,
        current_identifier: Option<&str>,
        content: &serde_json::Map<std::string::String, Value>,
    ) -> (
        Vec<ConfigurationOverride>,
        HashMap<String, Value>,
        Vec<String>,
    ) {
        let mut override_overrides = Vec::new();
        let mut override_content = HashMap::new();
        let mut override_keys = Vec::new();

        for (key, value) in content {
            if OVERRIDE_PROPERTY_REGEX.is_match(key) {
                override_overrides.extend(self.handle_override(key, value, current_identifier));
            } else {
                override_content.insert(key.clone(), value.clone());
                override_keys.push(key.clone());
            }
        }

        (override_overrides, override_content, override_keys)
    }
}

#[derive(Debug)]
pub struct Configuration {
    default_configuration: Arc<ConfigurationModel>,
    user_configuration: Arc<ConfigurationModel>,
    workspace_configuration: Arc<ConfigurationModel>,
    inmem_configuration: Arc<ConfigurationModel>,
    consolidated_configuration: ArcSwapOption<ConfigurationModel>,
}

impl Configuration {
    pub fn new(
        default_conf: Arc<ConfigurationModel>,
        user_conf: ConfigurationModel,
        workspace_conf: ConfigurationModel,
        inmem_conf: ConfigurationModel,
    ) -> Self {
        Configuration {
            default_configuration: default_conf,
            user_configuration: Arc::new(user_conf),
            workspace_configuration: Arc::new(workspace_conf),
            inmem_configuration: Arc::new(inmem_conf),
            consolidated_configuration: ArcSwapOption::from(None),
        }
    }

    pub fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value> {
        let consolidated_conf = self.get_consolidated_configuration(overrider_identifier);
        consolidated_conf.get_value(key).cloned()
    }

    pub fn get_consolidated_configuration(
        &self,
        overrider_identifier: Option<&str>,
    ) -> Arc<ConfigurationModel> {
        if let Some(config) = self.consolidated_configuration.load_full().as_ref() {
            if let Some(identifier) = overrider_identifier {
                return config.r#override(identifier.trim_start_matches('/'));
            }

            return Arc::clone(config);
        }

        let new_configuration = {
            let merged_configuration = self.default_configuration.merge(&[
                Arc::clone(&self.user_configuration),
                Arc::clone(&self.workspace_configuration),
                Arc::clone(&self.inmem_configuration),
            ]);

            Arc::new(merged_configuration)
        };

        self.consolidated_configuration
            .store(Some(Arc::clone(&new_configuration)));

        if let Some(identifier) = overrider_identifier {
            return new_configuration.r#override(identifier);
        }

        new_configuration
    }
}
