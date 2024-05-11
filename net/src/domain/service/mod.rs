pub mod config_service;
pub mod metric_service;
pub mod notification_service;
pub mod project_meta_service;
pub mod project_service;
pub mod session_service;

use analysis::policy_engine::PolicyEngine;
use fs::{fw::FileWatcher, real};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use crate::{config::Config, infra::adapter::sqlite::RootSQLiteAdapter};

use self::{
    config_service::ConfigService, metric_service::MetricService,
    notification_service::NotificationService, project_meta_service::ProjectMetaService,
    project_service::ProjectService, session_service::SessionService,
};

pub struct ServiceHub(
    pub Arc<ConfigService>,
    pub Arc<ProjectMetaService>,
    pub Arc<MetricService>,
    pub Arc<SessionService>,
    pub Arc<NotificationService>,
    pub Arc<RwLock<Option<ProjectService>>>,
);

impl ServiceHub {
    pub fn new(conf: &Config) -> Self {
        let b = bus::Bus::new();
        let realfs = Arc::new(real::FileSystem::new());
        let root_db = RootSQLiteAdapter::new(Arc::clone(&conf.conn));
        let fw = FileWatcher::new(b.clone());
        let pe = PolicyEngine::new(fw.clone(), b);

        ServiceHub(
            ConfigService::new(conf.preference.clone()),
            Arc::new(ProjectMetaService::new(
                realfs.clone(),
                root_db.project_meta_repo(),
            )),
            Arc::new(MetricService::new(Arc::new(pe))),
            Arc::new(SessionService::new(
                root_db.session_repo(),
                root_db.project_meta_repo(),
            )),
            Arc::new(NotificationService::new()),
            Arc::new(RwLock::new(None)),
        )
    }
}
