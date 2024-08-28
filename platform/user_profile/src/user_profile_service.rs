use anyhow::Result;
use std::path::PathBuf;

pub struct UserProfile {
    pub home: PathBuf,
    pub settings_resource: PathBuf,
}

#[async_trait]
pub trait UserProfileService<'a> {
    fn default_profile(&'a self) -> &'a UserProfile;
    async fn create_profile(&self, home: &PathBuf) -> Result<UserProfile>;
    async fn delete_profile(&self) -> Result<()>;
    async fn cleanup(&self) -> Result<()>;
}
