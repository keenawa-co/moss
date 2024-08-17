use std::sync::Arc;

use arc_swap::ArcSwap;
use hashbrown::HashMap;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::ConfigurationModel,
    configuration_registry::{ConfigurationRegistry, PropertyPolicy},
    policy::{PolicyDefinitionType, PolicyService},
};

pub struct ConfigurationPolicy<'a> {
    model: ArcSwap<ConfigurationModel>,
    registry: &'a ConfigurationRegistry<'a>,
    policy_service: ConfigurationPolicyService,
}

impl<'a> ConfigurationPolicy<'a> {
    pub fn new(
        registry: &'a ConfigurationRegistry<'a>,
        policy_service: ConfigurationPolicyService,
    ) -> Self {
        Self {
            model: ArcSwap::new(Arc::new(ConfigurationModel::empty())),
            registry,
            policy_service,
        }
    }

    pub fn initialize(&mut self, default_configuration: &DefaultConfiguration) {
        let default_configuration_model = default_configuration.get_configuration_model().unwrap(); // TODO: handle panic (should never happen)

        let mut configuration_model = ConfigurationModel::empty();

        for (property_key, property_policy) in
            self.find_all_model_policies(default_configuration_model)
        {
            if let Some(property_value) = self.policy_service.get_value(&property_policy.name) {
                configuration_model.set_value(property_key, property_value.clone());
            } else {
                // TODO: handle when value is None
            }
        }

        self.model.store(Arc::new(configuration_model));
    }

    pub fn get_model(&self) -> Arc<ConfigurationModel> {
        self.model.load_full()
    }

    fn find_all_model_policies(
        &self,
        model: Arc<ConfigurationModel>,
    ) -> HashMap<String, &PropertyPolicy> {
        let configuration_properties = self.registry.properties();
        let mut property_policies = HashMap::new();

        for key in model.get_attribute_names() {
            let property = if let Some(property) = configuration_properties.get(key) {
                property
            } else {
                continue;
            };

            if let Some(property_policy) = &property.schema.policy {
                // TODO: check for uniqueness and warning if the key already exists

                property_policies.insert(key.clone(), property_policy);
            }
        }

        property_policies
    }
}

pub struct ConfigurationPolicyService {
    pub definitions: HashMap<String, PolicyDefinitionType>,
    pub policies: HashMap<String, serde_json::Value>,
}

impl PolicyService for ConfigurationPolicyService {
    fn get_value(&self, name: impl ToString) -> Option<&serde_json::Value> {
        self.policies.get(&name.to_string())
    }
}
