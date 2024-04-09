use serde::de::DeserializeOwned;
use std::{fs, path::Path};

pub(super) fn load_toml_file<T: DeserializeOwned>(path: Box<Path>) -> anyhow::Result<T> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}
