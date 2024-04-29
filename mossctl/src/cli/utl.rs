use serde::de::DeserializeOwned;
use std::{fs, path::PathBuf};

// TODO: move to conf lib
pub(crate) fn load_toml_file<T: DeserializeOwned>(path: &PathBuf) -> anyhow::Result<T> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}
