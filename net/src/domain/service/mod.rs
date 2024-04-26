mod config_service;
mod metric_service;
mod portal_service;
mod project_service;

use std::sync::Arc;

pub use config_service::ConfigService;
pub use metric_service::MetricService;
pub use portal_service::PortalService;
pub use project_service::ProjectService;

pub struct ServiceLocator {
    pub config_service: Arc<ConfigService>,
    pub portal_service: Arc<PortalService>,
    pub project_service: Arc<ProjectService>,
    pub metric_service: Arc<MetricService>,
}
