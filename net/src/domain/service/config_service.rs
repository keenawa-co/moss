use std::sync::Arc;

use conf::pref::Preference;

#[derive(Clone, Debug)]
pub struct ConfigService {}

impl ConfigService {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}
