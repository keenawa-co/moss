use anyhow::Result;
use hashbrown::HashMap;
use serde_json::Value;
use std::{env, fs::File, io::Read};

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigurationEntryModel {
    contents: HashMap<String, serde_json::Value>,
    keys: Vec<String>,      // TODO: Consider using a HashSet
    overrides: Vec<String>, // TODO: Consider using a HashSet
}

impl Default for ConfigurationEntryModel {
    fn default() -> Self {
        Self {
            contents: Default::default(),
            keys: Default::default(),
            overrides: Default::default(),
        }
    }
}

impl ConfigurationEntryModel {
    pub fn get_value(&self, section: Option<&str>) -> Option<&serde_json::Value> {
        section.and_then(|s| self.contents.get(s))
    }

    pub fn merge(&self, others: Vec<ConfigurationEntryModel>) -> ConfigurationEntryModel {
        let mut merged = self.clone();
        for other in others {
            for (k, v) in other.contents {
                merged.contents.insert(k, v);
            }

            merged.keys.extend(other.keys);
            merged.overrides.extend(other.overrides)
        }

        merged
    }
}

// TODO: Use kernel/fs to work with the file system
pub struct ConfigurationModelParser;

impl ConfigurationModelParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_file(&self, file_path: &str) -> Result<ConfigurationEntryModel> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let json_map: HashMap<String, Value> = serde_json::from_str(&content)?;

        Ok(ConfigurationEntryModel {
            contents: json_map.clone(),
            keys: json_map.keys().cloned().collect(),
            overrides: vec!["<unimplemented>".to_string()],
        })
    }
}

pub struct ConfigurationModel {
    default_configuration: ConfigurationEntryModel,
    user_configuration: ConfigurationEntryModel,
    workspace_configuration: ConfigurationEntryModel,
    inmem_configuration: ConfigurationEntryModel,
}
