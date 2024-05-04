pub mod config_service;
pub mod metric_service;
pub mod project_meta_service;
pub mod project_service;
pub mod session_service;

use thiserror::Error;
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

// #[macro_export]
// macro_rules! not_found_2 {
//     ($msg:expr) => {{
//         $crate::domain::Error::Client(
//             $crate::domain::model::result::ClientError::NotFound(
//                 format!("{}:{}: {}", file!(), line!(), $msg),
//                 None
//             )
//         )
//     }};

//     ($fmt:expr, $($arg:expr),*) => {{
//         $crate::domain::Error::Client(
//             $crate::domain::model::result::ClientError::NotFound(
//                 format!("{}:{}: {}", file!(), line!(), format!($fmt, $($arg),+)),
//                 None
//             )
//         )
//     }};
// }

// #[derive(Error, Debug)]
// pub enum ServiceError {
//     #[error(transparent)]
//     Resource(ResourceError),

//     #[error(transparent)]
//     System(SystemError),
// }

// transparent_error!(System, SystemError::Database, sea_orm::DbErr);
// transparent_error!(System, SystemError::Anyhow, anyhow::Error);
// transparent_error!(System, SystemError::Notify, notify::Error);

// #[derive(Error, Debug)]
// pub enum ResourceError {
//     #[error("Cannot find the requested resource. {0}")]
//     NotFound(String),

//     #[error("Cannot or will not process the request. {0}")]
//     Invalid(String),
// }

// #[derive(Error, Debug)]
// pub enum SystemError {
//     #[error(transparent)]
//     Database(#[from] sea_orm::DbErr),

//     #[error(transparent)]
//     Anyhow(#[from] anyhow::Error),

//     #[error(transparent)]
//     Notify(#[from] notify::Error),
// }
