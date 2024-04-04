use moss_core::config::preference_file::BehaverPreferenceFile;

#[derive(Clone, Debug)]
pub struct UserService {
    pub user_settings: Box<BehaverPreferenceFile>,
}

impl UserService {
    pub fn init(user_settings: Box<BehaverPreferenceFile>) -> Self {
        Self { user_settings }
    }
}
