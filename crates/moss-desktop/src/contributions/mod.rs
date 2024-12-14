use crate::state::AppState;
use anyhow::Result;

pub mod layout_controls;
pub mod links;
pub mod resents;
pub mod workbench;

pub trait Contribution {
    fn contribute(app_state: &mut AppState) -> Result<()>;
}
