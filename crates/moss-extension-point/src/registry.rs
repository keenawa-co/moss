use anyhow::{anyhow, Result};
use arcstr::ArcStr;
use hashbrown::{HashMap, HashSet};
use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};

use crate::module::{
    extends::{ConfigurationDecl, ParameterValue},
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

pub struct ParameterProvider {
    pub id: String,
}

#[derive(Debug, Default)]
pub struct ConfigurationRegistry {
    parameter_decls: Vec<Arc<ConfigurationDecl>>,
    known_parameters: HashMap<ArcStr, Arc<ParameterValue>>,
    excluded_parameters: HashMap<ArcStr, Arc<ParameterValue>>,
    override_identifiers: HashSet<ArcStr>,
}

impl ConfigurationRegistry {
    pub fn parameters(&self) -> &HashMap<ArcStr, Arc<ParameterValue>> {
        &self.known_parameters
    }

    pub fn register(&mut self, decl: Arc<ConfigurationDecl>) {
        for (key, value) in &decl.parameters {
            if let Err(err) = self.validate_parameter(key, value) {
                warn!("Failed to register the parameter '{}': {err}", key);
                continue;
            }

            let target = if value.excluded {
                &mut self.excluded_parameters
            } else {
                &mut self.known_parameters
            };

            target.insert(ArcStr::clone(key), Arc::clone(value));
        }

        self.override_identifiers
            .extend(decl.overrides.keys().cloned());

        self.parameter_decls.push(decl);
    }

    fn validate_parameter(&self, key: &ArcStr, _parameter: &Arc<ParameterValue>) -> Result<()> {
        if key.trim().is_empty() {
            return Err(anyhow!("Cannot register a parameter with an empty key"));
        }

        // TODO: Validate the key against a regular expression.

        if self.known_parameters.get(key).is_some() {
            return Err(anyhow!("This parameter has already been registered"));
        }

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
            for ext in &module.extends {
                if let Some(configuration_decl) = &ext.configuration {
                    configurations.register(Arc::clone(configuration_decl));
                }
            }
        }

        Self {
            configurations: Arc::new(configurations),
        }
    }

    pub fn configuration_registry(&self) -> Arc<ConfigurationRegistry> {
        Arc::clone(&self.configurations)
    }
}
