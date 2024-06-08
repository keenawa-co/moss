pub mod config_service;
pub mod metric_service;
pub mod notification_service;
pub mod project_meta_service;
pub mod project_service;
pub mod session_service;
pub mod workspace_service;

use fs::real;
use std::sync::Arc;

use self::{
    config_service::ConfigService, metric_service::MetricService,
    notification_service::NotificationService, project_meta_service::ProjectMetaService,
    project_service::ProjectService, session_service::SessionService,
    workspace_service::WorkspaceService,
};

use super::port::rootdb::RootDbAdapter;

pub struct ServiceRoot<'a: 'static>(
    pub Arc<ConfigService>,
    pub Arc<ProjectMetaService>,
    pub Arc<MetricService>,
    pub Arc<SessionService>,
    pub Arc<NotificationService>,
    pub Arc<ProjectService<'a>>,
    pub Arc<WorkspaceService>,
);

impl<'a> ServiceRoot<'a> {
    pub fn new(rootdb: &impl RootDbAdapter) -> Arc<Self> {
        let realfs = Arc::new(real::FileSystem::new());

        Arc::new(ServiceRoot(
            ConfigService::new(),
            ProjectMetaService::new(realfs.clone(), rootdb.project_meta_repo()),
            MetricService::new(),
            SessionService::new(rootdb.session_repo(), rootdb.project_meta_repo()),
            NotificationService::new(),
            ProjectService::init(realfs.clone()),
            WorkspaceService::init(),
        ))
    }
}
