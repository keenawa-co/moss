use anyhow::Result;
use platform_core::context::Context;
use platform_fs::disk::file_system_service::AbstractDiskFileSystemService;
use std::{io::Read, path::PathBuf, sync::Arc};

use crate::{configuration_model::ConfigurationModel, configuration_parser::ConfigurationParser};

// TODO:
// - Use a LogService.
// - Use a PolicyService
pub struct UserSettings {
    parser: Arc<ConfigurationParser>,
    resource: PathBuf,

    fs_service: Arc<dyn AbstractDiskFileSystemService>,
}

impl<'a> UserSettings {
    pub fn new(
        file_path: PathBuf,
        content_parser: Arc<ConfigurationParser>,
        fs_service: Arc<dyn AbstractDiskFileSystemService>,
    ) -> Self {
        Self {
            parser: content_parser,
            resource: file_path,
            fs_service,
        }
    }

    pub async fn load_configuration(&self, ctx: &mut Context) -> Result<ConfigurationModel> {
        let mut file = self.fs_service.read_file(&self.resource).await?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        if content.trim().is_empty() {
            content = String::from("{}")
        }

        Ok(self.parser.parse(ctx, &content)?)
    }
}
