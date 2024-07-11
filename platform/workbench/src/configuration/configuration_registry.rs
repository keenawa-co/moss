use hashbrown::HashMap;
use serde_json::Value;

pub enum ConfigurationScope {
    Platform,
    Machine,
    Window,
    Resource,
}

pub enum ConfigurationNodeType {
    String,
    Bool,
    Number,
    Array,
    Object,
}

pub struct ConfigurationPropertySchema {
    pub scope: ConfigurationScope,
    pub order: Option<usize>,
    pub default: Option<Value>, // schema: schemars::schema::SchemaObject,
    pub description: Option<String>,
}

// type PropertiesDictionary = hashbrown::HashMap<String, >

pub struct ConfigurationNode {
    pub id: Option<String>,
    pub order: Option<usize>,
    pub typ: Option<ConfigurationNodeType>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub properties: HashMap<String, ConfigurationPropertySchema>,
}

pub struct ConfigurationDefaults {
    pub overrides: HashMap<String, Value>,
    pub source: Option<String>,
}

pub struct ConfigurationRegistry {
    registered_configuration_defaults: Vec<ConfigurationDefaults>,
    configuration_properties: HashMap<String, ConfigurationPropertySchema>,
}

impl ConfigurationRegistry {
    pub fn new() -> Self {
        Self {
            registered_configuration_defaults: Vec::new(),
            configuration_properties: HashMap::new(),
        }
    }

    pub fn register_configuration(&mut self, configuration: ConfigurationNode) {
        for (k, v) in configuration.properties.into_iter() {
            self.configuration_properties.insert(k, v);
        }
    }

    pub fn register_default_configurations(
        &mut self,
        default_configurations: Vec<ConfigurationDefaults>,
    ) {
        self.registered_configuration_defaults
            .extend(default_configurations);
        self.update_configuration_properties_with_defaults();
    }

    fn update_configuration_properties_with_defaults(&mut self) {
        for configuration_default in &self.registered_configuration_defaults {
            for (k, v) in &configuration_default.overrides {
                if let Some(property) = self.configuration_properties.get_mut(k) {
                    property.default = Some(v.clone())
                }
            }
        }
    }

    pub fn get_configuration_properties(&self) -> &HashMap<String, ConfigurationPropertySchema> {
        &self.configuration_properties
    }
}
