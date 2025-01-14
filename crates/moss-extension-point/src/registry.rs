use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use parking_lot::Mutex;
use serde_json::Value as JsonValue;
use std::{path::PathBuf, sync::Arc};

use crate::module::{
    configuration::{ConfigurationDecl, OverrideContext, ParameterValue},
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

#[derive(Debug)]
pub struct ValueProviderInfo {
    pub id: String,
}

#[derive(Debug, Default)]
pub struct DefaultOverrideDescriptor {
    value: JsonValue,
    provider_info: Option<ValueProviderInfo>,
}

#[derive(Debug, Default)]
pub struct DefaultOverrides {
    all: Vec<DefaultOverrideDescriptor>,
    consolidated: Option<DefaultOverrideDescriptor>,
}

#[derive(Debug, Default)]
pub struct ConfigurationRegistry {
    configuration_decls: Vec<(ArcStr, Arc<ConfigurationDecl>)>,
    known_parameters: HashMap<ArcStr, Arc<ParameterValue>>,
    excluded_parameters: HashMap<ArcStr, Arc<ParameterValue>>,
    overrides_by_context: HashMap<OverrideContext, HashMap<ArcStr, JsonValue>>,
    default_overrides: HashMap<ArcStr, HashMap<OverrideContext, DefaultOverrides>>,
    override_identifiers: HashSet<ArcStr>,
    decl_identifiers: HashSet<ArcStr>,
}

impl ConfigurationRegistry {
    pub fn parameters(&self) -> &HashMap<ArcStr, Arc<ParameterValue>> {
        &self.known_parameters
    }

    pub fn register(&mut self, decls: HashMap<ArcStr, Arc<ConfigurationDecl>>) {
        for (id, decl) in decls {
            if let Err(err) = self.validate_decl(&id, &decl) {
                warn!("Failed to register the parameter '{}': {err}", id);
                continue;
            }

            for (parameter_key, parameter_value) in &decl.parameters {
                if let Err(err) = self.validate_parameter(&parameter_key, &parameter_value) {
                    warn!(
                        "Failed to register the parameter '{}': {err}",
                        parameter_key
                    );
                    continue;
                }

                let target = if parameter_value.excluded {
                    &mut self.excluded_parameters
                } else {
                    &mut self.known_parameters
                };

                target.insert(ArcStr::clone(parameter_key), Arc::clone(parameter_value));
            }

            for (override_key, override_value) in &decl.overrides {
                // TODO: Validation

                for override_context in &override_value.context {
                    self.overrides_by_context
                        .entry(override_context.clone())
                        .or_insert_with(HashMap::new)
                        .insert(ArcStr::clone(override_key), override_value.value.clone());
                }
            }

            self.override_identifiers
                .extend(decl.overrides.keys().cloned());
            self.decl_identifiers.insert(ArcStr::clone(&id));
            self.configuration_decls.push((id, decl));
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

    fn validate_parameter(&self, key: &ArcStr, parameter: &Arc<ParameterValue>) -> Result<()> {
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

    fn validate_parameter_value(&self, _parameter: &Arc<ParameterValue>) -> Result<()> {
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
        }
    }

    pub fn configuration_registry(&self) -> Arc<ConfigurationRegistry> {
        Arc::clone(&self.configurations)
    }
}
