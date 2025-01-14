use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use parking_lot::Mutex;
use serde_json::Value as JsonValue;
use std::{marker::PhantomData, path::PathBuf, sync::Arc};

use crate::module::{
    configuration::{ConfigurationDecl, OverrideDecl, ParameterDecl},
    ExtensionPointModule,
};

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
    configuration_decls: Vec<(ArcStr, Arc<ConfigurationDecl>)>,
    known_parameters: HashMap<ArcStr, Arc<ParameterDecl>>,
    excluded_parameters: HashMap<ArcStr, Arc<ParameterDecl>>,
    default_overrides: HashMap<ArcStr, DefaultOverrides>,
    specific_overrides: HashMap<ArcStr, OverrideDecl>,
    override_identifiers: HashSet<ArcStr>,
    decl_identifiers: HashSet<ArcStr>,
}

impl ConfigurationRegistry {
    pub fn parameters(&self) -> &HashMap<ArcStr, Arc<ParameterDecl>> {
        &self.known_parameters
    }

    pub fn get_override(&self, key: &ArcStr) -> Option<Arc<DefaultOverrideDescriptor>> {
        self.default_overrides
            .get(key)
            .map(|value| &value.consolidated)
            .cloned()
    }

    pub fn register(&mut self, decls: HashMap<ArcStr, Arc<ConfigurationDecl>>) {
        for (id, decl) in decls {
            if let Err(err) = self.validate_decl(&id, &decl) {
                warn!("Failed to register the parameter '{}': {err}", id);
                continue;
            }

            self.register_parameters(&decl.parameters);
            self.register_overrides(&decl.overrides);

            self.override_identifiers
                .extend(decl.overrides.keys().cloned());
            self.decl_identifiers.insert(ArcStr::clone(&id));
            self.configuration_decls.push((id, decl));
        }
    }

    fn register_parameters(&mut self, parameters: &HashMap<ArcStr, Arc<ParameterDecl>>) {
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

    fn register_overrides(&mut self, overrides: &HashMap<ArcStr, Arc<OverrideDecl>>) {
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

    fn validate_decl(&self, key: &ArcStr, _decl: &Arc<ConfigurationDecl>) -> Result<()> {
        if self.decl_identifiers.get(key).is_some() {
            return Err(anyhow!(
                "A declaration with the identifier {} already exists.",
                key
            ));
        }

        self.validate_decl_key(key)?;

        Ok(())
    }

    fn validate_decl_key(&self, key: &ArcStr) -> Result<()> {
        if key.trim().is_empty() {
            return Err(anyhow!("Cannot register a parameter with an empty key"));
        }

        // TODO: Validate the key against a regular expression.

        Ok(())
    }

    fn validate_parameter(&self, key: &ArcStr, parameter: &Arc<ParameterDecl>) -> Result<()> {
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

    fn validate_parameter_value(&self, _parameter: &Arc<ParameterDecl>) -> Result<()> {
        // TODO: Validate the default value of the parameter to ensure it meets
        // the specified constraints. For example, if it is a numeric value, it must
        // not be less than the minimum or greater than the maximum, and so on.

        // TODO: Validate the default value to ensure it matches the specified type.
        // For example, if the type is `number`, but the default value is a string,
        // it should be flagged as invalid.

        Ok(())
    }
}

pub struct Registry<'a> {
    configurations: Arc<ConfigurationRegistry>,
    phantom: PhantomData<&'a ()>,
}

impl Registry<'_> {
    pub fn new(modules: &HashMap<PathBuf, ExtensionPointModule>) -> Self {
        let mut configurations = ConfigurationRegistry::default();

        for (_path, module) in modules {
            // OPTIMIZE: I think this can be optimized by removing cloning at this point.
            // In theory, after the data has been collected, there's no need to keep a
            // clone of the data in the Loader. Therefore, we could take ownership of
            // the data instead, eliminating the need for cloning here.
            configurations.register(module.configurations.clone());
        }

        Self {
            configurations: Arc::new(configurations),
            phantom: PhantomData,
        }
    }

    pub fn configuration_registry(&self) -> Arc<ConfigurationRegistry> {
        Arc::clone(&self.configurations)
    }
}
