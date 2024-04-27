use common::id::MNID;

use crate::domain::{self};

use super::project_service::ProjectSessionService;

pub struct ContextService {
    project_id: Option<MNID>,
    project_ss: Option<ProjectSessionService>,
}

impl Default for ContextService {
    fn default() -> Self {
        Self {
            project_id: None,
            project_ss: None,
        }
    }
}

impl ContextService {
    pub fn set_project(
        &mut self,
        project_id: MNID,
        project_ss: ProjectSessionService,
    ) -> domain::Result<()> {
        self.project_id = Some(project_id);
        self.project_ss = Some(project_ss);

        Ok(())
    }
}
