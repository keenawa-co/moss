pub mod workbench_tgui_contrib;

use std::{borrow::BorrowMut, cell::RefCell, path::PathBuf, sync::Arc};

use anyhow::Result;
use platform_configuration_common::{
    attribute_name, configuration_policy::ConfigurationPolicyService,
    configuration_registry::ConfigurationRegistry, AbstractConfigurationService,
};
use platform_formation_common::service_registry::ServiceRegistry;
use specta::Type;
use workbench_service_configuration_tgui::configuration_service::WorkspaceConfigurationService;
use workbench_tgui_contrib::WORKBENCH_TGUI_WINDOW;
use workspace::{Workspace, WorkspaceId};

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Type, Serialize)]
pub enum WorkbenchState {
    Empty,
    Workspace,
}

pub struct Workbench<'a> {
    workspace_id: WorkspaceId,
    service_registry: ServiceRegistry,
    configuration_registry: Arc<ConfigurationRegistry<'a>>,
}

impl<'a> Workbench<'a> {
    pub fn new(service_registry: ServiceRegistry, workspace_id: WorkspaceId) -> Result<Self> {
        let mut configuration_registry = Arc::new(ConfigurationRegistry::new());

        // configuration_registry
        //     .borrow_mut()
        //     .register_configuration(WORKBENCH_TGUI_WINDOW.clone());

        Ok(Self {
            workspace_id,
            service_registry,
            configuration_registry,
        })
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.initialize_services()?;

        let config_service = self
            .service_registry
            .get_unchecked::<WorkspaceConfigurationService>();

        let value = config_service.get_value(attribute_name!(editor.fontSize));
        println!("Value `editor.fontSize` form None: {:?}", value);

        Ok(())
    }

    fn initialize_services(&mut self) -> Result<()> {
        let workspace = self.restore_workspace();

        let configuration_policy_service = ConfigurationPolicyService {
            definitions: {
                use platform_configuration_common::policy::PolicyDefinitionType;

                let mut this = hashbrown::HashMap::new();

                this.insert(
                    "editorLineHeightPolicy".to_string(),
                    PolicyDefinitionType::Number,
                );

                this
            },
            policies: {
                let mut this = hashbrown::HashMap::new();
                this.insert(
                    "editorLineHeightPolicy".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(45)),
                );

                this
            },
        };

        let workspace_configuration_service = WorkspaceConfigurationService::new(
            workspace,
            Arc::clone(&self.configuration_registry),
            configuration_policy_service,
        );

        self.service_registry
            .insert(workspace_configuration_service);

        Ok(())
    }

    fn restore_workspace(&self) -> Workspace {
        match &self.workspace_id {
            WorkspaceId::Empty => Workspace {
                id: WorkspaceId::Empty,
                folders: vec![],
                configuration_uri: None,
            },
            WorkspaceId::Some(_id) => {
                struct SimpleWorkspaceData {
                    path: PathBuf,
                }

                // TODO: This data should be obtained from the storage service
                // and represent the project from the previous session.
                let mock_workspace_data = SimpleWorkspaceData {
                    path: PathBuf::from(format!(".moss/settings.json")),
                };

                Workspace {
                    id: self.workspace_id.clone(),
                    folders: vec![],
                    configuration_uri: Some(mock_workspace_data.path),
                }
            }
        }
    }

    pub fn get_state(&self) -> WorkbenchState {
        WorkbenchState::Empty
    }
}
