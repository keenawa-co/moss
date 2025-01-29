use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use moss_mel::foundations::configuration::{ConfigurationNode, Override, Parameter};
use parking_lot::Mutex;
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};

static __EP_REGISTRY__: Mutex<Vec<PathBuf>> = Mutex::new(vec![]);

#[macro_export]
macro_rules! submit {
    ($path:expr) => {
        #[$crate::ctor::ctor]
        fn __submit__() {
            $crate::registry::with_mut(|registry| {
                registry.push(std::path::PathBuf::from($path));
            });
        }
    };
}

pub fn take() -> Vec<PathBuf> {
    std::mem::take(&mut *__EP_REGISTRY__.lock())
}

pub fn with_mut(f: impl FnOnce(&mut Vec<PathBuf>)) {
    f(&mut __EP_REGISTRY__.lock())
}

#[derive(Debug, Clone)]
pub struct ValueProviderInfo {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct DefaultOverrideDescriptor {
    pub value: JsonValue,
    pub provider_info: Option<ValueProviderInfo>,
}

#[derive(Debug)]
pub struct DefaultOverrides {
    all: Vec<Arc<DefaultOverrideDescriptor>>,
    consolidated: Arc<DefaultOverrideDescriptor>,
}

#[derive(Debug, Default)]
pub struct ConfigurationRegistry {
    configuration_nodes: Vec<Arc<ConfigurationNode>>,
    known_parameters: HashMap<ArcStr, Arc<Parameter>>,
    excluded_parameters: HashMap<ArcStr, Arc<Parameter>>,
    default_overrides: HashMap<ArcStr, DefaultOverrides>,
    specific_overrides: HashMap<ArcStr, Override>,
    override_identifiers: HashSet<ArcStr>,
    decl_identifiers: HashSet<ArcStr>,
}

impl ConfigurationRegistry {
    pub fn parameters(&self) -> &HashMap<ArcStr, Arc<Parameter>> {
        &self.known_parameters
    }

    pub fn get_override(&self, key: &ArcStr) -> Option<Arc<DefaultOverrideDescriptor>> {
        self.default_overrides
            .get(key)
            .map(|value| &value.consolidated)
            .cloned()
    }

    pub fn register<I>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = ConfigurationNode>,
    {
        for node in nodes {
            if let Err(err) = self.validate_decl(&node) {
                warn!("Failed to register the parameter '{}': {err}", node.ident);
                continue;
            }

            self.register_parameters(&node.parameters);
            self.register_overrides(&node.overrides);

            self.override_identifiers
                .extend(node.overrides.keys().cloned());
            self.decl_identifiers.insert(ArcStr::clone(&node.ident));
            self.configuration_nodes.push(Arc::new(node));
        }
    }

    fn register_parameters(&mut self, parameters: &HashMap<ArcStr, Arc<Parameter>>) {
        for (key, decl) in parameters {
            if let Err(err) = self.validate_parameter(&key, &decl) {
                warn!("Failed to register the parameter '{}': {err}", key);
                continue;
            }

            let target = if decl.excluded {
                &mut self.excluded_parameters
            } else {
                &mut self.known_parameters
            };

            target.insert(ArcStr::clone(key), Arc::clone(decl));
        }
    }

    fn register_overrides(&mut self, overrides: &HashMap<ArcStr, Arc<Override>>) {
        for (override_key, override_decl) in overrides {
            // TODO: validate the override key and declaration

            if override_decl.value.is_null() {
                warn!("The value of the '{override_key}' override is null. This override will be ignored.");
                continue;
            }

            if override_decl.context.is_some() {
                // TODO: context specific handling can be added here in the future
                continue;
            }

            let new_descriptor = Arc::new(match &override_decl.value {
                JsonValue::Bool(_) | JsonValue::Number(_) | JsonValue::String(_) => {
                    DefaultOverrideDescriptor {
                        value: override_decl.value.clone(),
                        provider_info: None,
                    }
                }
                JsonValue::Null => unreachable!("Null values are already checked earlier."),
                _ => unimplemented!("Handling for this type is not implemented."),
            });

            let key = ArcStr::from(override_key);

            self.default_overrides
                .entry(key)
                .and_modify(|default_overrides| {
                    default_overrides.all.push(Arc::clone(&new_descriptor));
                    default_overrides.consolidated = Arc::clone(&new_descriptor);
                })
                .or_insert_with(|| DefaultOverrides {
                    all: vec![Arc::clone(&new_descriptor)],
                    consolidated: new_descriptor,
                });
        }
    }

    fn validate_decl(&self, decl: &ConfigurationNode) -> Result<()> {
        let key = decl.ident.clone();
        if self.decl_identifiers.get(&key).is_some() {
            return Err(anyhow!(
                "A declaration with the identifier {} already exists.",
                key
            ));
        }

        self.validate_decl_key(&key)?;

        Ok(())
    }

    fn validate_decl_key(&self, key: &ArcStr) -> Result<()> {
        if key.trim().is_empty() {
            return Err(anyhow!("Cannot register a parameter with an empty key"));
        }

        // TODO: Validate the key against a regular expression.

        Ok(())
    }

    fn validate_parameter(&self, key: &ArcStr, parameter: &Parameter) -> Result<()> {
        if self.known_parameters.get(key).is_some() {
            return Err(anyhow!("This parameter has already been registered"));
        }

        self.validate_parameter_key(key)?;
        self.validate_parameter_value(parameter)?;

        Ok(())
    }

    fn validate_parameter_key(&self, key: &ArcStr) -> Result<()> {
        if key.trim().is_empty() {
            return Err(anyhow!("Cannot register a parameter with an empty key"));
        }

        // TODO: Validate the key against a regular expression.

        Ok(())
    }

    fn validate_parameter_value(&self, _parameter: &Parameter) -> Result<()> {
        // TODO: Validate the default value of the parameter to ensure it meets
        // the specified constraints. For example, if it is a numeric value, it must
        // not be less than the minimum or greater than the maximum, and so on.

        // TODO: Validate the default value to ensure it matches the specified type.
        // For example, if the type is `number`, but the default value is a string,
        // it should be flagged as invalid.

        Ok(())
    }
}

pub struct Registry {
    configurations: Arc<ConfigurationRegistry>,
}

impl Registry {
    pub fn new(configurations: ConfigurationRegistry) -> Self {
        // let mut configurations = ConfigurationRegistry::default();

        // configurations.register(scope.configurations.into_values());

        Self {
            configurations: Arc::new(configurations),
        }
    }

    pub fn configuration_registry(&self) -> Arc<ConfigurationRegistry> {
        Arc::clone(&self.configurations)
    }
}
