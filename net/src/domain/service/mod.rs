mod config_service;
mod metric_service;
mod project_meta_service;
mod project_service;
mod session_service;

use tokio::sync::RwLock;

pub use config_service::ConfigService;
pub use metric_service::MetricService;
pub use project_meta_service::ProjectMetaService;
pub use project_service::ProjectService;
pub use session_service::SessionService;

pub struct ServiceLocator {
    pub config_service: ConfigService,
    pub project_meta_service: ProjectMetaService,
    pub project_service: RwLock<Option<ProjectService>>,
    pub metric_service: MetricService,
    pub session_service: RwLock<SessionService>,
}
