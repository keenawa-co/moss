use anyhow::Context;
use std::path::PathBuf;

pub mod locale_service;
pub mod theme_service;
pub(crate) fn get_home_dir() -> anyhow::Result<PathBuf> {
    dirs::home_dir().context("Home directory not found!")
}
