pub mod service;
pub mod tgui;

#[macro_use]
extern crate lazy_static;

pub enum WorkbenchState {
    Empty,
    Workspace,
}
