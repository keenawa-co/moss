use anyhow::Result;
use platform_configuration_common::{
    attribute_name, configuration_service::ConfigurationService, AbstractConfigurationService,
};
use platform_formation_common::service_group::ServiceGroup;
use specta::Type;

#[macro_use]
extern crate serde;

#[derive(Debug, Type, Serialize)]
pub enum WorkbenchState {
    Empty,
    Workspace,
}

pub struct Workbench {}

impl Workbench {
    pub fn new(service_group: ServiceGroup) -> Result<Self> {
        let config_service = service_group.get_unchecked::<ConfigurationService>();

        let value = config_service.get_value(attribute_name!(editor.fontSize));
        println!("Value `editor.fontSize` form None: {:?}", value);

        Ok(Self {})
    }

    pub fn get_state(&self) -> WorkbenchState {
        WorkbenchState::Empty
    }
}
