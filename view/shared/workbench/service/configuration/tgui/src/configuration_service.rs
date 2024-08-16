use std::{path::PathBuf, sync::Arc};

use anyhow::Context;
use anyhow::Result;
use platform_configuration_common::{
    configuration_default::DefaultConfiguration,
    configuration_model::{
        AttributeName, ConfigurationModel, ConfigurationParser, UserConfiguration,
    },
    configuration_policy::{ConfigurationPolicy, ConfigurationPolicyService},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};
use workbench_service_configuration_common::configuration_model::WorkspaceConfiguration;
use workspace::Workspace;

pub struct WorkspaceConfigurationService {
    workspace: Workspace,
    configuration: WorkspaceConfiguration,
}

impl WorkspaceConfigurationService {
    pub fn new(
        workspace: Workspace,
        registry: Arc<ConfigurationRegistry>,
        policy_service: ConfigurationPolicyService,
    ) -> Self {
        let parser = ConfigurationParser::new(Arc::clone(&registry)); // TODO: platform ConfigurationParser?

        let default_configuration = DefaultConfiguration::new(Arc::clone(&registry)); // TODO: use WorkspaceDefaultConfiguration
        default_configuration.initialize();

        // TODO: use UserDataProfileService
        let config_file_path = &std::path::PathBuf::from("C:/Users/semjonov/.moss/settings.json");
        let user_configuration = UserConfiguration::new(config_file_path, Arc::new(parser));
        let user_configuration_model = ConfigurationModel::empty();
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")
            .unwrap();

        let mut configuration_policy =
            ConfigurationPolicy::new(Arc::clone(&registry), policy_service);
        configuration_policy.initialize(&default_configuration);

        let policy_configuration_model = configuration_policy.get_model();

        Self {
            workspace,
            configuration: WorkspaceConfiguration::new(
                default_configuration_model,
                policy_configuration_model,
                Arc::new(user_configuration_model),
                Arc::new(ConfigurationModel::empty()),
                Arc::new(ConfigurationModel::empty()),
            ),
        }
    }
}

#[async_trait]
impl AbstractConfigurationService for WorkspaceConfigurationService {
    fn get_value(&self, attribute_name: AttributeName) -> Option<serde_json::Value> {
        self.configuration.get_value(&attribute_name)
    }

    async fn update_value(
        &self,
        attribute_name: AttributeName,
        value: &serde_json::Value,
    ) -> Result<()> {
        todo!()
    }
}
