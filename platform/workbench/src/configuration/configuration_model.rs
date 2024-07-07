use anyhow::Result;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde_json::Value;
use std::{fs::File, io::Read, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationEntryModel {
    pub contents: HashMap<String, serde_json::Value>,
    pub keys: Vec<String>,
    pub overrides: HashMap<String, HashMap<String, Value>>,
}

impl ConfigurationEntryModel {
    pub fn empty() -> Self {
        Self {
            contents: HashMap::new(),
            keys: Vec::new(),
            overrides: HashMap::new(),
        }
    }

    pub fn get_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.contents.get(key)
    }

    fn override_configuration(&self, identifier: &str) -> Self {
        if let Some(override_contents) = self.overrides.get(identifier) {
            let mut new_contents = self.contents.clone();
            let mut new_keys = self.keys.clone();

            Self::merge_content(&mut new_contents, override_contents);

            for key in override_contents.keys() {
                if !new_keys.contains(key) {
                    new_keys.push(key.clone());
                }
            }

            ConfigurationEntryModel {
                contents: new_contents,
                keys: new_keys,
                overrides: self.overrides.clone(),
            }
        } else {
            self.clone()
        }
    }

    fn merge(&self, others: Vec<ConfigurationEntryModel>) -> Self {
        let mut merged = self.clone();
        for other in others {
            Self::merge_content(&mut merged.contents, &other.contents);
            let new_keys: Vec<String> = other
                .keys
                .into_iter()
                .filter(|key| !merged.keys.contains(key))
                .collect();
            merged.keys.extend(new_keys);
            merged.overrides.extend(other.overrides);
        }
        merged
    }
}

impl ConfigurationEntryModel {
    fn merge_content(target: &mut HashMap<String, Value>, source: &HashMap<String, Value>) {
        for (key, value) in source {
            target.insert(key.clone(), value.clone());
        }
    }
}

// TODO: Use kernel/fs to work with the file system
pub struct ConfigurationModelParser;

impl ConfigurationModelParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_file(&self, file_path: &str) -> Result<ConfigurationEntryModel> {
        let re_override_property = regex!(r#"^\[.*\]$"#);

        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json_map: HashMap<String, Value> = serde_json::from_str(&content)?;

        Ok(ConfigurationEntryModel {
            contents: json_map.clone(),
            keys: json_map.keys().cloned().collect(),
            overrides: HashMap::new(), // FIXME: not implemented
        })
    }
}

pub struct ConfigurationModel {
    pub default_configuration: ConfigurationEntryModel,
    pub user_configuration: ConfigurationEntryModel,
    pub workspace_configuration: ConfigurationEntryModel,
    pub inmem_configuration: ConfigurationEntryModel,

    pub consolidated_configuration: RwLock<Option<ConfigurationEntryModel>>,
}

impl ConfigurationModel {
    pub fn new(
        default_conf: ConfigurationEntryModel,
        user_conf: ConfigurationEntryModel,
        workspace_conf: ConfigurationEntryModel,
        inmem_conf: ConfigurationEntryModel,
    ) -> Self {
        ConfigurationModel {
            default_configuration: default_conf,
            user_configuration: user_conf,
            workspace_configuration: workspace_conf,
            inmem_configuration: inmem_conf,
            consolidated_configuration: RwLock::new(None),
        }
    }

    pub fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value> {
        let consolidated_conf = self.get_consolidated_configuration(overrider_identifier);
        consolidated_conf.get_value(key).cloned()
    }

    pub fn get_consolidated_configuration(
        &self,
        overrider_identifier: Option<&str>,
    ) -> Arc<ConfigurationEntryModel> {
        {
            let read_guard = self.consolidated_configuration.read();
            if let Some(ref config) = *read_guard {
                if let Some(identifier) = overrider_identifier {
                    return Arc::new(config.clone().override_configuration(identifier));
                }

                return Arc::new(config.clone());
            }
        }

        let mut write_guard = self.consolidated_configuration.write();
        if write_guard.is_none() {
            let merged_configuration = self.default_configuration.merge(vec![
                self.user_configuration.clone(),
                self.workspace_configuration.clone(),
                self.inmem_configuration.clone(),
            ]);
            *write_guard = Some(merged_configuration);
        }

        if let Some(identifier) = overrider_identifier {
            return Arc::new(
                write_guard
                    .as_ref()
                    .unwrap()
                    .clone()
                    .override_configuration(identifier),
            );
        }

        Arc::new(write_guard.as_ref().unwrap().clone())
    }
}
