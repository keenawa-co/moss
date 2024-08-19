use anyhow::Context;
use anyhow::Result;
use platform_configuration::{
    configuration_default::DefaultConfiguration,
    configuration_model::{
        AttributeName, ConfigurationModel, ConfigurationParser, UserConfiguration,
    },
    configuration_policy::{ConfigurationPolicy, ConfigurationPolicyService},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};
use platform_fs::disk::file_system_service::AbstractDiskFileSystemService;
use platform_workspace::Workspace;
use std::{path::PathBuf, sync::Arc};
use workbench_service_configuration_common::configuration_model::WorkspaceConfiguration;

pub struct WorkspaceConfigurationService {
    workspace: Workspace,
    configuration: WorkspaceConfiguration,
}

impl WorkspaceConfigurationService {
    pub async fn new<'a>(
        workspace: Workspace,
        registry: &'a ConfigurationRegistry<'_>,
        policy_service: ConfigurationPolicyService,
        user_configuration_resource: &PathBuf,
        fs_service: Arc<dyn AbstractDiskFileSystemService>,
    ) -> Self {
        let parser = ConfigurationParser::new(&registry); // TODO: platform ConfigurationParser?

        let default_configuration = DefaultConfiguration::new(&registry); // TODO: use WorkspaceDefaultConfiguration
        default_configuration.initialize();

        let user_configuration =
            UserConfiguration::new(user_configuration_resource, Arc::new(parser), fs_service);
        let user_configuration_model = user_configuration
            .load_configuration()
            .await
            .context("failed to load user configuration model")
            .unwrap();
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")
            .unwrap();

        let mut configuration_policy = ConfigurationPolicy::new(&registry, policy_service);
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
