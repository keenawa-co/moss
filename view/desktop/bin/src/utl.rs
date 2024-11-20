use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use workbench_desktop::window::NativeWindowConfiguration;
use workbench_service_environment_tao::environment_service::NativeEnvironmentService;

pub struct MockStorageService {}

impl MockStorageService {
    fn new() -> Self {
        Self {}
    }
}

pub fn create_service_registry(
    native_window_configuration: NativeWindowConfiguration,
) -> Result<ServiceRegistry> {
    let mut service_registry = ServiceRegistry::new();

    let mock_storage_service = MockStorageService::new();

    let fs_service = DiskFileSystemService::new();
    let environment_service =
        NativeEnvironmentService::new(native_window_configuration.home_dir.clone());

    service_registry.insert(mock_storage_service);
    service_registry.insert(environment_service);
    service_registry.insert(Arc::new(fs_service));

    Ok(service_registry)
}

pub fn get_home_dir() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        windows_home_dir()
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        unix_home_dir()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

/// Retrieves the home directory on Unix-like systems (macOS and Linux).
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn unix_home_dir() -> Result<PathBuf, String> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .map_err(|e| format!("Failed to retrieve HOME environment variable: {}", e))
}

/// Retrieves the home directory on Windows.
#[cfg(target_os = "windows")]
fn windows_home_dir() -> Result<PathBuf, String> {
    match homedir::my_home() {
        Ok(result) => result.ok_or("Home directory not found".to_string()),
        Err(e) => Err(format!("Failed to retrieve HOME directory: {}", e)),
    }
}

pub fn get_themes_dir() -> Result<PathBuf, String> {
    let home_dir = get_home_dir()?;
    Ok(home_dir
        .join(".config")
        .join("moss")
        .join("themes"))
}