mod config_service;
mod metric_service;
mod portal_service;
mod project_service;
mod session_project_service;
mod session_service;

use tokio::sync::RwLock;

pub use config_service::ConfigService;
pub use metric_service::MetricService;
pub use portal_service::PortalService;
pub use project_service::ProjectService;
pub use session_project_service::SessionProjectService;
pub use session_service::SessionService;

pub struct ServiceLocator {
    pub config_service: ConfigService,
    pub portal_service: PortalService,
    pub project_service: ProjectService,
    pub metric_service: MetricService,
    pub session_service: RwLock<SessionService>,
    pub session_project_service: RwLock<Option<SessionProjectService>>,
}
