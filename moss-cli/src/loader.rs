use std::{fs, sync::Arc};

use moss_core::config::preference::Preference;

pub fn load_preference_file(path: String) -> anyhow::Result<Arc<Preference>> {
    let content = fs::read_to_string(path)?;
    let preference_file: Preference = toml::from_str(&content)?;

    Ok(Arc::new(preference_file))
}
