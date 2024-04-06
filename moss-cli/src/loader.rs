use std::fs;

use moss_core::config::preference::Preference;

pub fn load_preference_file(path: String) -> anyhow::Result<Box<Preference>> {
    let content = fs::read_to_string(path)?;
    let preference_file: Preference = toml::from_str(&content)?;

    Ok(Box::new(preference_file))
}
