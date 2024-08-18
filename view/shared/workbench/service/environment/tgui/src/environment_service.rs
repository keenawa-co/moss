use std::path::PathBuf;

use platform_environment::environment_model::Environment;

pub struct NativeEnvironmentService {
    user_home_dir: PathBuf,
    platform_environment: Environment,
}

impl NativeEnvironmentService {
    pub fn new(home_dir: &PathBuf) -> Self {
        let config_dir = home_dir.join(".config").join("moss");

        let platform_environment = Environment {
            untitled_workspaces_cache_dir: config_dir.join("untitled-workspaces"),
            workspaces_cache_dir: config_dir.join("workspaces"),
            file_history_dir: config_dir.join("history"),
            cache_dir: config_dir.join("cache"),
            // argv_resource: config_dir.join("workspaces"),
            log_level: "debug".to_string(),
            log_dir: config_dir.join("logs"),
        };

        Self {
            user_home_dir: home_dir.clone(),
            platform_environment,
        }
    }

    pub fn user_home_dir(&self) -> &PathBuf {
        &self.user_home_dir
    }
}
