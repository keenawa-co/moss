use hashbrown::HashMap;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum ConfigurationScope {
    Platform,
    Machine,
    Window,
    Resource,
}

#[derive(Debug, Clone)]
pub enum ConfigurationNodeType {
    String,
    Bool,
    Number,
    Array,
    Object,
}

#[derive(Debug, Clone)]
pub struct ConfigurationPropertySchema {
    pub scope: ConfigurationScope,
    pub r#type: ConfigurationNodeType,
    pub order: Option<usize>,
    pub default: Option<Value>, // schema: schemars::schema::SchemaObject,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationNode {
    pub id: Option<String>,
    pub order: Option<usize>,
    pub r#type: Option<ConfigurationNodeType>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub properties: HashMap<String, ConfigurationPropertySchema>,
    pub all_of: Option<Vec<ConfigurationNode>>,
}

#[derive(Debug, Clone)]
pub struct ConfigurationDefaults {
    pub overrides: HashMap<String, Value>,
    pub source: Option<String>,
}

#[derive(Debug)]
pub struct ConfigurationRegistry {
    registered_configuration_defaults: Vec<ConfigurationDefaults>,
    configuration_properties: HashMap<String, ConfigurationPropertySchema>,
    configuration_contributors: Vec<ConfigurationNode>,

    // NOTE: think about moving this into structure SchemaStorage ... or smth
    all_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    platform_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    machine_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    window_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    resource_settings_schema: HashMap<String, ConfigurationPropertySchema>,
}

impl ConfigurationRegistry {
    pub fn new() -> Self {
        Self {
            registered_configuration_defaults: Vec::new(),
            configuration_properties: HashMap::new(),
            configuration_contributors: Vec::new(),

            all_settings_schema: HashMap::new(),
            platform_settings_schema: HashMap::new(),
            machine_settings_schema: HashMap::new(),
            window_settings_schema: HashMap::new(),
            resource_settings_schema: HashMap::new(),
        }
    }

    pub fn get_configuration_properties(&self) -> &HashMap<String, ConfigurationPropertySchema> {
        &self.configuration_properties
    }

    pub fn register_configuration(&mut self, configuration: ConfigurationNode) {
        self.do_configuration_registration(configuration);

        // TODO: emit event
    }

    pub fn register_default_configurations(
        &mut self,
        default_configurations: Vec<ConfigurationDefaults>,
    ) {
        self.registered_configuration_defaults
            .extend(default_configurations);
        self.update_configuration_properties_with_defaults();
    }

    fn register_json_configuration(&mut self, configuration: &ConfigurationNode) {
        for (key, property) in &configuration.properties {
            self.update_schema(key, property);
        }

        // for sub_node in &configuration.all_of {
        //     self.register_json_configuration(configuration)
        // }
    }

    fn update_schema(&mut self, key: &str, property: &ConfigurationPropertySchema) {
        self.all_settings_schema
            .insert(key.to_string(), property.clone());

        match property.scope {
            ConfigurationScope::Platform => {
                self.platform_settings_schema
                    .insert(key.to_string(), property.clone());
            }
            ConfigurationScope::Machine => {
                self.machine_settings_schema
                    .insert(key.to_string(), property.clone());
            }
            ConfigurationScope::Window => {
                self.window_settings_schema
                    .insert(key.to_string(), property.clone());
            }
            ConfigurationScope::Resource => {
                self.resource_settings_schema
                    .insert(key.to_string(), property.clone());
            }
        }
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

    fn do_configuration_registration(&mut self, configuration: ConfigurationNode) {
        self.validate_and_register_properties(&configuration);
        self.register_json_configuration(&configuration);
        self.configuration_contributors.push(configuration);
    }

    fn validate_and_register_properties(&mut self, configuration: &ConfigurationNode) {
        // TODO: move registration logic into do_configuration_registration fn
        for (key, property) in &configuration.properties {
            self.configuration_properties
                .insert(key.clone(), property.clone());

            // TODO: insert into configuration schema
        }

        if let Some(sub_node) = &configuration.all_of {
            for node in sub_node {
                self.validate_and_register_properties(node);
            }
        }
    }
}
