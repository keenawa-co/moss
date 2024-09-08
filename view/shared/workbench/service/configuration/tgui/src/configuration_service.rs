use anyhow::Context as ResultContext;
use anyhow::Result;
use platform_configuration::configuration_parser::ConfigurationParser;
use platform_configuration::user_settings::UserSettings;
use platform_configuration::{
    configuration_default::DefaultConfiguration,
    configuration_model::{AttributeName, ConfigurationModel},
    configuration_policy::{ConfigurationPolicy, ConfigurationPolicyService},
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};
use platform_core::context::entity::Model;
use platform_core::context::Context;
use platform_fs::disk::file_system_service::AbstractDiskFileSystemService;
use platform_workspace::Workspace;
use std::{path::PathBuf, sync::Arc};
use workbench_service_configuration_common::configuration_model::WorkspaceConfiguration;

pub struct WorkspaceConfigurationService {
    workspace: Workspace,
    configuration: WorkspaceConfiguration,
}

impl WorkspaceConfigurationService {
    pub async fn new(
        ctx: &mut Context,
        workspace: Workspace,
        registry: Model<ConfigurationRegistry>,
        policy_service: ConfigurationPolicyService,
        user_configuration_resource: PathBuf,
        fs_service: Arc<dyn AbstractDiskFileSystemService>,
    ) -> Self {
        let parser = ConfigurationParser::new(registry.clone()); // TODO: platform ConfigurationParser?

        let default_configuration = DefaultConfiguration::new(registry.clone()); // TODO: use WorkspaceDefaultConfiguration
        default_configuration.initialize(ctx);

        let user_configuration =
            UserSettings::new(user_configuration_resource, Arc::new(parser), fs_service);
        let user_configuration_model = user_configuration
            .load_configuration(ctx)
            .await
            .context("failed to load user configuration model")
            .unwrap();
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")
            .unwrap();

        let mut configuration_policy = ConfigurationPolicy::new(registry.clone(), policy_service);
        configuration_policy.initialize(ctx, &default_configuration);

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
