use std::fs;

use mosscore::config::preference_file::BehaverPreferenceFile;

pub fn load_behaver_preference_file(path: String) -> anyhow::Result<Box<BehaverPreferenceFile>> {
    let content = fs::read_to_string(path)?;
    let preference_file: BehaverPreferenceFile = toml::from_str(&content)?;

    Ok(Box::new(preference_file))
}
