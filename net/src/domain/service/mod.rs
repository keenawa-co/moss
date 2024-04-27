mod config_service;
mod context_service;
mod metric_service;
mod portal_service;
mod project_service;

use std::sync::RwLock;

pub use config_service::ConfigService;
pub use context_service::ContextService;
pub use metric_service::MetricService;
pub use portal_service::PortalService;
pub use project_service::ProjectService;
pub use project_service::ProjectSessionService;

pub struct ServiceLocator {
    pub config_service: ConfigService,
    pub portal_service: PortalService,
    pub project_service: ProjectService,
    pub metric_service: MetricService,
    pub context_service: RwLock<ContextService>,
}
