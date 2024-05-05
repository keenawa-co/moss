pub mod config_service;
pub mod metric_service;
pub mod project_meta_service;
pub mod project_service;
pub mod session_service;

use tokio::sync::RwLock;

use self::{
    config_service::ConfigService, metric_service::MetricService,
    project_meta_service::ProjectMetaService, project_service::ProjectService,
    session_service::SessionService,
};

pub struct ServiceLocator {
    pub config_service: ConfigService,
    pub project_meta_service: ProjectMetaService,
    pub project_service: RwLock<Option<ProjectService>>,
    pub metric_service: MetricService,
    pub session_service: RwLock<SessionService>,
}
