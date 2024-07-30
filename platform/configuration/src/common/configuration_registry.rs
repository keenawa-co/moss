use std::sync::Arc;

use hashbrown::{HashMap, HashSet};
use lazy_regex::Regex as LazyRegex;
use serde_json::Value;

type Regex = LazyRegex;

/// Enumeration representing the scope of a configuration setting.
/// This enum defines the different levels at which a configuration setting can be applied.
#[derive(Debug, Clone)]
pub enum ConfigurationScope {
    /// Application-specific configuration that applies globally across the entire platform.
    Application,
    /// Machine-specific configuration that applies to the entire machine.
    Machine,
    /// Window-specific configuration that applies to a single window within the application.
    Window,
    /// Resource-specific configuration that applies to individual resources, such as files or projects.
    Resource,
}

impl Default for ConfigurationScope {
    fn default() -> Self {
        Self::Window
    }
}

/// Enumeration representing the type of a configuration setting.
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigurationNodeType {
    Null,
    String,
    Bool,
    Number,
    Array,
    Object,
}

impl Default for ConfigurationNodeType {
    fn default() -> Self {
        Self::Null
    }
}

impl ConfigurationNodeType {
    pub fn default_value(r#type: &Self) -> serde_json::Value {
        match r#type {
            ConfigurationNodeType::Null => Value::Null,
            ConfigurationNodeType::String => Value::String(String::new()),
            ConfigurationNodeType::Bool => Value::Bool(false),
            ConfigurationNodeType::Number => Value::Number(serde_json::Number::from(0)),
            ConfigurationNodeType::Array => Value::Array(vec![]),
            ConfigurationNodeType::Object => Value::Object(serde_json::Map::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SourceInfo {
    pub id: String,
    pub display_name: Option<String>,
}

/// A struct representing a configuration key with optional overrides.
#[derive(Debug, PartialEq, Eq)]
struct Key {
    /// A set of overrides for specific contexts.
    override_for: HashSet<String>,
    /// The identifier string for the key, potentially with sub-identifiers.
    ident: String,
}

impl Key {
    fn parse(s: &str) -> Result<Self, String> {
        let mut overrides = HashSet::new();
        let mut remaining = s;

        while remaining.starts_with('[') {
            if let Some(end) = remaining.find(']') {
                let override_ = &remaining[1..end];
                overrides.insert(override_.to_string());
                remaining = &remaining[end + 1..];
            } else {
                return Err("Mismatched brackets in override section".to_string());
            }
        }

        if remaining.starts_with('.') {
            remaining = &remaining[1..];
        }

        Ok(Key {
            override_for: overrides,
            ident: remaining.to_string(),
        })
    }
}

/// A macro to create a `Key` struct with optional overrides and sub-identifiers.
///
/// # Examples
///
/// Basic usage with no overrides:
///
/// ```rust
/// let key = key!(editor.fontSize);
/// assert_eq!(key.override_for.is_empty(), true);
/// assert_eq!(key.ident, "editor.fontSize");
/// ```
///
/// Usage with a single override:
///
/// ```rust
/// let key = key!([rust].editor.fontSize);
/// assert!(key.override_for.contains("rust"));
/// assert_eq!(key.ident, "editor.fontSize");
/// ```
///
/// Usage with multiple overrides:
///
/// ```rust
/// let key = key!([typescript][javascript].editor.fontSize);
/// assert!(key.override_for.contains("typescript"));
/// assert!(key.override_for.contains("javascript"));
/// assert_eq!(key.ident, "editor.fontSize");
/// ```
#[macro_export]
macro_rules! key {
    // Collect overrides and construct the Key with sub-identifiers
    (@collect_overrides [$($override:ident)+] $ident:ident $(. $subident:ident)*) => {{
        let mut overrides = HashSet::new();
        $(
            overrides.insert(stringify!($override).to_string());
        )+
        let ident = concat!(stringify!($ident), $(concat!(".", stringify!($subident))),*).to_string();
        Key {
            override_for: overrides,
            ident,
        }
    }};

    // Handle a single override
    ([$override:ident] . $($tail:tt)*) => {
        key!(@collect_overrides [$override] $($tail)*)
    };

    // Handle multiple overrides
    ([$first:ident] $([$rest:ident])+ . $($tail:tt)*) => {
        key!(@collect_overrides [$first $($rest)+] $($tail)*)
    };

    // Handle the case without overrides and with sub-identifiers
    ($ident:ident $(. $subident:ident)*) => {{
        let overrides: HashSet<String> = HashSet::new();
        let ident = concat!(stringify!($ident), $(concat!(".", stringify!($subident))),*).to_string();
        Key {
            override_for: overrides,
            ident,
        }
    }};
}

pub trait Keyable: ToString {
    fn as_straight_key(&self) -> Option<String> {
        Some(self.to_string())
    }

    fn as_composite_key(&self) -> Option<CompositeKey>;
}

impl Keyable for String {
    fn as_composite_key(&self) -> Option<CompositeKey> {
        None
    }
}

impl Keyable for &str {
    fn as_composite_key(&self) -> Option<CompositeKey> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CompositeKey {
    pub override_id: String,
    pub key: String,
}

impl ToString for CompositeKey {
    fn to_string(&self) -> String {
        format!("[{}].{}", self.override_id, self.key)
    }
}

impl Keyable for CompositeKey {
    fn as_composite_key(&self) -> Option<CompositeKey> {
        Some(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct PropertyMap {
    table: HashMap<String, ConfigurationPropertySchema>,
    overrides: HashSet<String>,
}

impl Default for PropertyMap {
    fn default() -> Self {
        Self {
            table: Default::default(),
            overrides: Default::default(),
        }
    }
}

impl IntoIterator for PropertyMap {
    type Item = (String, ConfigurationPropertySchema);
    type IntoIter = hashbrown::hash_map::IntoIter<String, ConfigurationPropertySchema>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.into_iter()
    }
}

impl<'a> IntoIterator for &'a PropertyMap {
    type Item = (&'a String, &'a ConfigurationPropertySchema);
    type IntoIter = hashbrown::hash_map::Iter<'a, String, ConfigurationPropertySchema>;

    fn into_iter(self) -> Self::IntoIter {
        self.table.iter()
    }
}

impl PropertyMap {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            overrides: HashSet::new(),
        }
    }

    pub fn get_overrides(&self) -> &HashSet<String> {
        &self.overrides
    }

    pub fn extend(&mut self, item: PropertyMap) {
        self.overrides.extend(item.overrides);
        self.table.extend(item.table);
    }

    pub fn insert(&mut self, key: impl Keyable, value: ConfigurationPropertySchema) {
        self.table.insert(key.to_string(), value);

        if let Some(composite_key) = key.as_composite_key() {
            self.overrides.insert(composite_key.override_id);
        }
    }
}

#[derive(Debug, Clone)]
pub enum StringPresentationFormatType {
    Multiline,
    Singleline,
}

#[derive(Debug, Clone)]
pub struct PropertyPolicy {
    pub name: String,
}

/// Struct representing a schema for a configuration property.
/// This struct defines the metadata and constraints for a configuration setting.
#[derive(Debug, Clone)]
pub struct ConfigurationPropertySchema {
    /// Unique identifier for the property.
    pub id: Option<String>,
    /// The scope of the configuration property, indicating the level at which it applies.
    pub scope: Option<ConfigurationScope>,
    /// The type of the configuration property, specifying the kind of value it holds.
    pub typ: Option<ConfigurationNodeType>,
    /// The order in which the configuration property appears in the settings UI.
    pub order: Option<usize>,
    /// The default value of the configuration property, if any.
    pub default: Option<Value>,
    /// A description of the configuration property, providing context and usage information.
    pub description: Option<String>,
    /// Indicates if the configuration property is protected from contributions by extensions.
    /// If true, the property cannot be overridden by contributions.
    pub protected_from_contribution: Option<bool>,
    /// Specifies if the configuration property is allowed only for restricted sources.
    /// If true, the property can only be set by trusted sources.
    pub allow_for_only_restricted_source: Option<bool>,
    /// Indicates if the configuration property is included in the registry.
    /// If false, the property is excluded from the configuration registry.
    pub schemable: Option<bool>,
    /// Indicates that a property is deprecated.
    pub deprecated: Option<bool>,
    /// Tags associated with the property:
    /// - For filtering
    /// - Use `experimental` to mark property as experimental.
    /// - Use `deprecated` to mark property as deprecated.
    /// - Use `beta` to mark property that are in beta testing.
    pub tags: Option<String>,

    pub policy: Option<PropertyPolicy>,

    /// Minimum number of properties in the schema.
    pub max_properties: Option<usize>,
    /// Minimum number of properties in the schema.
    pub min_properties: Option<usize>,

    /// Elements of the array defined by the schema.
    pub array_items: Option<Value>,
    /// Minimum number of items in the array.
    pub array_min_items: Option<usize>,
    /// Maximum number of items in the array.
    pub array_max_items: Option<usize>,
    /// Indicates whether the items in the array must be unique.
    pub array_unique_items: Option<bool>,

    /// Pattern that the string must match.
    pub string_pattern: Option<Regex>,
    /// Minimum length of the string.
    pub string_min_length: Option<usize>,
    /// Maximum length of the string.
    pub string_max_length: Option<usize>,
    /// Specifies the string settings format, defaults to `Singleline` if unspecified.
    pub string_presentation_format: Option<StringPresentationFormatType>,

    /// Minimum value for numbers.
    pub number_min_value: Option<isize>,
    /// Maximum value for numbers.
    pub number_max_value: Option<isize>,

    /// Allowed values for a property.
    pub enum_items: Option<Value>,
    /// Labels for enum items
    pub enum_item_labels: Option<Vec<String>>,
}

impl Default for ConfigurationPropertySchema {
    fn default() -> Self {
        let default_default_value =
            ConfigurationNodeType::default_value(&ConfigurationNodeType::Null);

        Self {
            id: None,
            scope: Some(ConfigurationScope::Window),
            typ: Some(ConfigurationNodeType::Null),
            order: None,
            default: Some(default_default_value),
            description: None,
            protected_from_contribution: Some(false),
            allow_for_only_restricted_source: Some(false),
            schemable: Some(true),
            deprecated: Some(false),
            tags: None,
            policy: None,
            max_properties: None,
            min_properties: None,
            array_items: None,
            array_min_items: None,
            array_max_items: None,
            array_unique_items: None,
            string_pattern: None,
            string_min_length: Some(0),
            string_max_length: Some(255),
            string_presentation_format: Some(StringPresentationFormatType::Singleline),
            number_min_value: None,
            number_max_value: None,
            enum_items: None,
            enum_item_labels: None,
        }
    }
}

/// Struct representing a configuration node.
#[derive(Debug, Clone)]
pub struct ConfigurationNode {
    /// The ID of the configuration node.
    pub id: String,
    /// The scope of the configuration property, indicating the level at which it applies.
    pub scope: Option<ConfigurationScope>,
    /// The order in which the configuration node appears.
    pub order: Option<usize>,
    /// The type of the configuration node.
    pub r#type: Option<ConfigurationNodeType>,
    /// The title of the configuration node.
    pub title: Option<String>,
    /// The description of the configuration node.
    pub description: Option<String>,
    /// The properties of the configuration node.
    pub properties: Option<PropertyMap>,
    /// Sub-nodes of the configuration node.
    pub parent_of: Option<Vec<ConfigurationNode>>,

    pub source: Option<SourceInfo>,
}

/// Struct representing default configurations.
#[derive(Debug, Clone)]
pub struct ConfigurationDefaults {
    /// The default values for configuration properties.
    /// This field contains a map where the keys are configuration property names and the values are their default values.
    /// These defaults can override the initial values specified in the configuration schema.
    pub overrides: HashMap<String, Value>,
    /// The source of the default configurations.
    /// This optional field indicates the origin of these default configurations, such as an extension or a specific configuration context.
    /// It provides context for the default values and helps track their origin.
    pub source: Option<SourceInfo>,
}

#[derive(Debug, Clone)]
pub struct RegisteredConfigurationPropertySchema {
    pub schema: Arc<ConfigurationPropertySchema>,
    pub source: Option<SourceInfo>,
}

impl RegisteredConfigurationPropertySchema {
    pub fn is_protected(&self) -> bool {
        self.schema.protected_from_contribution.unwrap_or(false)
    }
}

impl RegisteredConfigurationPropertySchema {
    fn new(property: ConfigurationPropertySchema, source: Option<SourceInfo>) -> Self {
        let registered_property = Self {
            schema: Arc::new(property),
            source,
        };

        registered_property
    }
}

/// Struct representing an override value for a default configuration.
#[derive(Debug, Clone)]
pub struct ConfigurationDefaultOverrideValue {
    /// The value of the override.
    /// This field stores the new default value for the configuration property.
    /// It replaces the original default value defined in the configuration schema.
    pub value: Value,
    /// The source of the override.
    /// This optional field indicates the origin of the override, such as an extension or user-defined configuration.
    /// It helps track where the override came from and provides context for the overridden value.
    pub source: Option<SourceInfo>,
}

/// Struct to store schema information for configuration settings.
#[derive(Debug)]
pub struct ConfigurationSchemaStorage {
    /// Schema for all settings.
    all_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for platform-specific settings.
    platform_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for machine-specific settings.
    machine_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for window-specific settings.
    window_settings_schema: HashMap<String, ConfigurationPropertySchema>,
    /// Schema for resource-specific settings.
    resource_settings_schema: HashMap<String, ConfigurationPropertySchema>,
}

impl ConfigurationSchemaStorage {
    fn empty() -> Self {
        Self {
            all_settings_schema: HashMap::new(),
            platform_settings_schema: HashMap::new(),
            machine_settings_schema: HashMap::new(),
            window_settings_schema: HashMap::new(),
            resource_settings_schema: HashMap::new(),
        }
    }

    fn update_schema(&mut self, key: &str, property: &ConfigurationPropertySchema) {
        self.all_settings_schema
            .insert(key.to_string(), property.clone());

        match property
            .scope
            .as_ref()
            .unwrap_or(&ConfigurationScope::Window)
        {
            ConfigurationScope::Application => {
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
}

/// Registry to manage configurations and their schemas.
#[derive(Debug)]
pub struct ConfigurationRegistry {
    #[allow(unused)] // Designed for future expansion
    /// List of registered default configurations.
    /// These configurations define standard default values for various settings that can be
    /// overridden by users or other configurations.
    registered_configuration_defaults: Vec<ConfigurationDefaults>,

    #[allow(unused)] // Designed for future expansion
    /// Map of configuration default overrides.
    /// This hashmap stores overridden default values for configuration properties, indexed by their keys.
    /// Overrides can come from different sources and can change the default values defined in `registered_configuration_defaults`.
    configuration_defaults_overrides: HashMap<String, ConfigurationDefaultOverrideValue>,

    /// Map of configuration properties.
    /// This hashmap stores the properties of configurations, indexed by their keys.
    /// Each property includes metadata such as type, scope, default values, and descriptions.
    configuration_properties: HashMap<String, RegisteredConfigurationPropertySchema>,

    /// List of configuration nodes contributed.
    /// This map contains all configuration nodes that have been registered to the registry.
    /// Configuration nodes can include multiple properties and sub-nodes.
    configuration_contributors: HashMap<String, Arc<ConfigurationNode>>,

    /// Set of override identifiers.
    /// This set contains identifiers that are used to specify configurations that can override default values.
    /// Override identifiers are used to create specialized settings for different scopes or contexts.
    override_identifiers: HashSet<String>,

    /// Storage for configuration schemas.
    /// This structure stores the schema definitions for all settings, organized by their scope (e.g., platform, machine, window, resource).
    /// It is used to generate and manage the JSON schema for configuration properties.
    configuration_schema_storage: ConfigurationSchemaStorage,

    /// Map of excluded configuration properties.
    /// This hashmap stores properties that are explicitly excluded from the configuration registry.
    /// These properties are not included in the configuration schema and are not available for users to configure.
    excluded_configuration_properties: HashMap<String, RegisteredConfigurationPropertySchema>,
}

impl ConfigurationRegistry {
    pub fn new() -> Self {
        Self {
            registered_configuration_defaults: Vec::new(),
            configuration_properties: HashMap::new(),
            configuration_contributors: HashMap::new(),
            configuration_defaults_overrides: HashMap::new(),
            override_identifiers: HashSet::new(),
            configuration_schema_storage: ConfigurationSchemaStorage::empty(),
            excluded_configuration_properties: HashMap::new(),
        }
    }

    pub fn get_configuration_properties(
        &self,
    ) -> &HashMap<String, RegisteredConfigurationPropertySchema> {
        &self.configuration_properties
    }

    pub fn get_excluded_configuration_properties(
        &self,
    ) -> &HashMap<String, RegisteredConfigurationPropertySchema> {
        &self.excluded_configuration_properties
    }

    pub fn get_override_identifiers(&self) -> &HashSet<String> {
        &self.override_identifiers
    }

    pub fn register_configuration(&mut self, configuration: ConfigurationNode) {
        self.configuration_contributors
            .insert(configuration.id.clone(), Arc::new(configuration.clone()));
        self.register_json_configuration(&configuration);

        let _properties = self.do_configuration_registration(&configuration, false);

        // TODO: Emit schema change events
    }

    fn do_configuration_registration(
        &mut self,
        configuration: &ConfigurationNode,
        validate: bool,
    ) -> PropertyMap {
        let node_scope_or_default = configuration
            .scope
            .as_ref()
            .unwrap_or(&ConfigurationScope::Window);

        let mut node_properties = configuration
            .properties
            .clone()
            .unwrap_or(PropertyMap::new());

        // TODO: validate incoming override identifiers before extend
        self.override_identifiers
            .extend(node_properties.overrides.clone());

        for (key, property) in &node_properties {
            if validate && !self.validate_property(&property) {
                continue;
            }

            let mut property_schema = property.clone();

            if node_properties.overrides.get(key).is_some() {
                // Assigning a specific scope is redundant since this property already implies a particular context.
                property_schema.scope = None;
            } else {
                property_schema.scope = Some(node_scope_or_default.clone());
                property_schema.allow_for_only_restricted_source =
                    Some(property.allow_for_only_restricted_source.unwrap_or(false));
            }

            let registered_property = RegisteredConfigurationPropertySchema::new(
                property_schema,
                configuration.source.clone(),
            );

            if property.schemable.unwrap_or(true) {
                self.configuration_properties
                    .insert(key.clone(), registered_property);
            } else {
                self.excluded_configuration_properties
                    .insert(key.clone(), registered_property);
            }
        }

        if let Some(sub_nodes) = configuration.parent_of.as_ref() {
            sub_nodes.iter().for_each(|node| {
                let sub_properties = self.do_configuration_registration(node, false);
                node_properties.extend(sub_properties.clone());
                self.register_json_configuration(&node);
            });
        }

        node_properties
    }

    fn validate_property(&self, _property: &ConfigurationPropertySchema) -> bool {
        unimplemented!()
    }

    fn register_json_configuration(&mut self, configuration: &ConfigurationNode) {
        if let Some(properties) = &configuration.properties {
            for (key, property) in properties {
                if property.schemable.unwrap_or(true) {
                    self.configuration_schema_storage
                        .update_schema(key, property);
                }
            }
        }

        for sub_node in configuration.parent_of.as_ref().unwrap_or(&vec![]) {
            self.register_json_configuration(sub_node);
        }
    }

    pub fn register_default_configurations(
        &mut self,
        default_configurations: Vec<ConfigurationDefaults>,
    ) {
        let _properties = self.do_register_default_configuration(default_configurations);

        // TODO: Emit schema change events
        unimplemented!()
    }

    fn do_register_default_configuration(
        &mut self,
        _default_configurations: Vec<ConfigurationDefaults>,
    ) -> HashSet<String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_with_multiple_overrides_and_subidentifiers() {
        let key = key!([typescript][javascript].editor.fontSize);
        let mut expected_overrides = HashSet::new();
        expected_overrides.insert("typescript".to_string());
        expected_overrides.insert("javascript".to_string());
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor.fontSize".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_key_with_single_override_and_subidentifiers() {
        let key = key!([rust].editor.fontSize);
        let mut expected_overrides = HashSet::new();
        expected_overrides.insert("rust".to_string());
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor.fontSize".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_key_with_multiple_subidentifiers() {
        let key = key!(editor.fontSize.lineHeight);
        let expected_overrides = HashSet::new();
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor.fontSize.lineHeight".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_key_with_single_identifier() {
        let key = key!(editor);
        let expected_overrides = HashSet::new();
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_multiple_overrides_and_subidentifiers() {
        let s = "[typescript][javascript].editor.fontSize";
        let key = Key::parse(s).unwrap();
        let mut expected_overrides = HashSet::new();
        expected_overrides.insert("typescript".to_string());
        expected_overrides.insert("javascript".to_string());
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor.fontSize".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_single_override_and_subidentifiers() {
        let s = "[rust].editor.fontSize";
        let key = Key::parse(s).unwrap();
        let mut expected_overrides = HashSet::new();
        expected_overrides.insert("rust".to_string());
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor.fontSize".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_multiple_subidentifiers() {
        let s = "editor.fontSize.lineHeight";
        let key = Key::parse(s).unwrap();
        let expected_key = Key {
            override_for: HashSet::new(),
            ident: "editor.fontSize.lineHeight".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_single_identifier() {
        let s = "editor";
        let key = Key::parse(s).unwrap();
        let expected_key = Key {
            override_for: HashSet::new(),
            ident: "editor".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_multiple_overrides_single_subidentifier() {
        let s = "[typescript][javascript].editor";
        let key = Key::parse(s).unwrap();
        let mut expected_overrides = HashSet::new();
        expected_overrides.insert("typescript".to_string());
        expected_overrides.insert("javascript".to_string());
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_single_override_single_identifier() {
        let s = "[rust].editor";
        let key = Key::parse(s).unwrap();
        let mut expected_overrides = HashSet::new();
        expected_overrides.insert("rust".to_string());
        let expected_key = Key {
            override_for: expected_overrides,
            ident: "editor".to_string(),
        };
        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_parse_with_invalid_format() {
        let s = "[rust.editor.fontSize";
        let result = Key::parse(s);
        assert!(result.is_err());
    }
}
