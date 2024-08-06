use std::sync::Arc;

use crate::common::workspace_configuration_model::WorkspaceConfiguration;
use configuration::common::configuration_model::ConfigurationModel;
use workspace::Workspace;

pub struct WorkspaceService {
    workspace: Workspace,
    configuration: WorkspaceConfiguration,
    initialized: bool,
}

impl WorkspaceService {
    pub fn new(
        default_configuration: Arc<ConfigurationModel>,
        policy_configuration: Arc<ConfigurationModel>,
        user_configuration: Arc<ConfigurationModel>,
        workspace_configuration: Arc<ConfigurationModel>,
        inmem_configuration: Arc<ConfigurationModel>,
    ) -> Self {
        todo!()
    }
}
