pub mod config_service;
pub mod metric_service;
pub mod notification_service;
pub mod project_meta_service;
pub mod project_service;
pub mod session_service;
pub mod workspace_service;

use fs::real;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{config::Config, infra::adapter::sqlite::RootSQLiteAdapter};

use self::{
    config_service::ConfigService, metric_service::MetricService,
    notification_service::NotificationService, project_meta_service::ProjectMetaService,
    project_service::ProjectService, session_service::SessionService,
    workspace_service::WorkspaceService,
};

pub struct ServiceRoot(
    pub Arc<ConfigService>,
    pub Arc<ProjectMetaService>,
    pub Arc<MetricService>,
    pub Arc<SessionService>,
    pub Arc<NotificationService>,
    pub Arc<ProjectService<'static>>,
    pub Arc<WorkspaceService>,
);

impl ServiceRoot {
    pub fn new(conf: &Config) -> Self {
        let realfs = Arc::new(real::FileSystem::new());
        let root_db = RootSQLiteAdapter::new(Arc::clone(&conf.conn));

        // let project_service = Arc::new(ProjectService::init(realfs.clone()));

        ServiceRoot(
            ConfigService::new(conf.preference.clone()),
            Arc::new(ProjectMetaService::new(
                realfs.clone(),
                root_db.project_meta_repo(),
            )),
            Arc::new(MetricService::new()),
            Arc::new(SessionService::new(
                root_db.session_repo(),
                root_db.project_meta_repo(),
            )),
            Arc::new(NotificationService::new()),
            Arc::new(ProjectService::init(realfs.clone())),
            Arc::new(WorkspaceService::init()),
        )
    }
}
