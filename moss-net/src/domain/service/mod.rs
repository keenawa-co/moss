mod config_service;
mod portal_service;

use std::sync::Arc;

pub use config_service::ConfigService;
pub use portal_service::PortalService;

pub struct ServiceLocator {
    pub config_service: Arc<ConfigService>,
    pub portal_service: Arc<PortalService>,
}
