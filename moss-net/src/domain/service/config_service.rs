use std::sync::Arc;

use moss_core::config::preference::Preference;

#[derive(Clone, Debug)]
pub struct ConfigService {
    pub preferences: Arc<Preference>,
}

impl ConfigService {
    pub fn new(preferences: Arc<Preference>) -> Self {
        Self { preferences }
    }
}
