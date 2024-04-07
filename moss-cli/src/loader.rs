use serde::de::DeserializeOwned;
use std::fs;

pub fn load_toml_file<T: DeserializeOwned>(path: String) -> anyhow::Result<T> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}
