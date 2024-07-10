pub mod configuration;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_regex;

pub enum WorkbenchState {
    Empty,
    Workspace,
}
