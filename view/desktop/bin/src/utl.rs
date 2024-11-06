use std::sync::Arc;

use anyhow::Result;
use platform_formation::service_registry::ServiceRegistry;
use platform_fs::disk::file_system_service::DiskFileSystemService;
use platform_workspace::WorkspaceId;
use serde_json::Value;
use workbench_desktop::window::NativeWindowConfiguration;
use workbench_service_environment_tao::environment_service::NativeEnvironmentService;

pub struct MockStorageService {}

struct SimpleWindowState {
    workspace_id: WorkspaceId,
}

impl MockStorageService {
    fn new() -> Self {
        Self {}
    }

    fn get_last_window_state(&self) -> SimpleWindowState {
        SimpleWindowState {
            workspace_id: WorkspaceId::Some("workspace_path_hash".to_string()),
        }
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
