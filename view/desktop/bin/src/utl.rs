use anyhow::{Context as _, Result};
use std::path::PathBuf;

// use platform_formation::service_registry::ServiceRegistry;
// use platform_fs::disk::file_system_service::DiskFileSystemService;

// pub struct MockStorageService {}

// impl MockStorageService {
//     fn new() -> Self {
//         Self {}
//     }
// }

// pub fn create_service_registry(
//     native_window_configuration: NativeWindowConfiguration,
// ) -> Result<ServiceRegistry> {
//     let mut service_registry = ServiceRegistry::new();

//     let mock_storage_service = MockStorageService::new();

//     let fs_service = DiskFileSystemService::new();
//     let environment_service =
//         NativeEnvironmentService::new(native_window_configuration.home_dir.clone());

//     service_registry.insert(mock_storage_service);
//     service_registry.insert(environment_service);
//     service_registry.insert(Arc::new(fs_service));

//     Ok(service_registry)
// }

pub fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().context("Home directory not found!")
}

pub fn get_themes_dir() -> Result<PathBuf> {
    Ok(get_home_dir()?.join(".config").join("moss").join("themes"))
}
