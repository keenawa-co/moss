pub mod layout_controls;
pub mod links;
pub mod resents;
pub mod workbench;

use anyhow::Result;

use crate::state::AppState;

pub trait ContributionOld {
    fn contribute(app_state: &mut AppState) -> Result<()>;
}
