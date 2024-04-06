use moss_core::config::behaver_preference::BehaverPreferenceConfig;

#[derive(Clone, Debug)]
pub struct ConfigService {
    pub preferences: Box<BehaverPreferenceConfig>,
}

impl ConfigService {
    pub fn new(preferences: Box<BehaverPreferenceConfig>) -> Self {
        Self { preferences }
    }
}
