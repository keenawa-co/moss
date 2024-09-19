use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::path::Path;

#[derive(Deserialize, Default)]
pub struct ConfigFile {
    pub licenses: Vec<String>,
    pub rust_workspace_audit: RustWorkspaceAuditBlock,
}

#[derive(Deserialize, Default)]
pub struct RustWorkspaceAuditBlock {
    pub ignore: HashMap<String, Vec<String>>,
}

impl ConfigFile {
    pub async fn load<P: AsRef<Path> + Debug>(config_path: P) -> Result<Self> {
        let content_str = match smol::fs::read_to_string(&config_path).await {
            Ok(content) => content,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Err(anyhow!("File {:?} is not found", config_path));
            }
            Err(e) => return Err(anyhow!(e)),
        };

        Ok(toml::from_str(&content_str)?)
    }
}
