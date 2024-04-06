use moss_core::config::preference::Preference;

#[derive(Clone, Debug)]
pub struct ConfigService {
    pub preferences: Box<Preference>,
}

impl ConfigService {
    pub fn new(preferences: Box<Preference>) -> Self {
        return Self { preferences };
    }
}
