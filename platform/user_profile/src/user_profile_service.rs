use std::path::PathBuf;

use crate::UserProfile;

pub trait UserProfileService<'a> {
    type Error;

    fn default_profile(&'a self) -> &'a UserProfile;
    fn create_profile(&self, home: PathBuf) -> Result<UserProfile, Self::Error>;
    fn delete_profile(&self) -> Result<(), Self::Error>;
    fn cleanup(&self) -> Result<(), Self::Error>;
}
