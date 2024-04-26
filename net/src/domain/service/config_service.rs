use std::sync::Arc;

use conf::pref::Preference;

#[derive(Clone, Debug)]
pub struct ConfigService {
    pub preferences: Arc<Preference>,
}

impl ConfigService {
    pub fn new(preferences: Arc<Preference>) -> Self {
        Self { preferences }
    }
}
