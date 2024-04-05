use std::fs;

use moss_core::config::behaver_preference::BehaverPreferenceConfig;

pub fn load_behaver_preference_file(path: String) -> anyhow::Result<Box<BehaverPreferenceConfig>> {
    let content = fs::read_to_string(path)?;
    let preference_file: BehaverPreferenceConfig = toml::from_str(&content)?;

    Ok(Box::new(preference_file))
}
