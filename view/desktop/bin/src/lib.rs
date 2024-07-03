mod mem;

pub mod menu;
pub mod service;

use service::project_service::ProjectService;
use service::session_service::SessionService;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

pub struct AppState {
    pub project_service: ProjectService,
    pub session_service: SessionService,
}
