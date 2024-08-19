use platform_user_profile::user_profile_service::UserProfileService as PlatformUserProfileService;
use platform_user_profile::UserProfile;
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

// TODO: use fs service
pub struct UserProfileService {
    default_profile: UserProfile,
}

impl UserProfileService {
    pub fn new(home_dir: PathBuf) -> io::Result<Self> {
        let config_dir = home_dir.join(".config").join("moss");

        let settings_resource = config_dir
            .join("user")
            .join("default")
            .join("settings.json");
        if !settings_resource.exists() {
            if let Some(parent) = settings_resource.parent() {
                fs::create_dir_all(parent)?;
            }

            File::create(&settings_resource)?;
        }

        Ok(Self {
            default_profile: UserProfile {
                home: home_dir,
                settings_resource,
            },
        })
    }
}

impl<'a> PlatformUserProfileService<'a> for UserProfileService {
    type Error = io::Error;

    fn default_profile(&'a self) -> &'a UserProfile {
        &self.default_profile
    }

    fn create_profile(&self, home: std::path::PathBuf) -> Result<UserProfile, Self::Error> {
        todo!()
    }

    fn delete_profile(&self) -> Result<(), Self::Error> {
        todo!()
    }

    fn cleanup(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
