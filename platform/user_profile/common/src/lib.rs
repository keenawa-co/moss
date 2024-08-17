pub mod user_profile_service;

use std::path::PathBuf;

pub struct UserProfile {
    pub home: PathBuf,
    pub settings_resource: PathBuf,
}
