use anyhow::{Context as _, Result};
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("Home directory not found!")
}

pub fn get_themes_dir() -> Result<PathBuf> {
    Ok(get_home_dir()?.join(".config").join("moss").join("themes"))
}
