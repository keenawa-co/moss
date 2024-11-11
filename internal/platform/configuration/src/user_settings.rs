use anyhow::Result;
use platform_core::context_v2::Context;
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

    pub fn load_configuration(&self, ctx: &mut Context) -> Result<ConfigurationModel> {
        // When the file doesn't exist, the code should not panic
        // Instead, it should treat it as if the config file is empty
        let mut file = ctx.block_on_with(self.fs_service.read_file(&self.resource));
        let mut content = String::new();

        match file {
            Ok(ref mut file) => {file.read_to_string(&mut content)?;},
            Err(_) => {content = String::from("{}");}
        }

        if content.trim().is_empty() {
            content = String::from("{}")
        }

        Ok(self.parser.parse(ctx, &content)?)
    }
}
