mod cli;
mod config;
mod migration;

pub use cli::init;

#[macro_use]
extern crate tracing;
