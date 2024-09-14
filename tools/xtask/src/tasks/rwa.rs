use smol::fs;
use std::{collections::HashMap, io};

use anyhow::{anyhow, Result};
use clap::Parser;
use serde::Deserialize;
use toml::Value;

use tracing::{error, info, warn};

use cargo_metadata::Metadata;

#[derive(Parser)]
pub struct WorkspaceAuditCommandArgs {}

#[derive(Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    rust_workspace_audit: RustWorkspaceAuditBlock,
}

#[derive(Deserialize, Default)]
struct RustWorkspaceAuditBlock {
    #[serde(default)]
    ignore: HashMap<String, Vec<String>>,
}

pub async fn run_rwa(_args: WorkspaceAuditCommandArgs, workspace: Metadata) -> Result<()> {
    // FIXME:
    let ignored_deps = load_ignored_dependencies("config.toml").await?;

    for package in workspace.workspace_packages() {
        info!("analyzing '{}'...", package.name);
        let cargo_toml_content = fs::read_to_string(&package.manifest_path).await?;
        let cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

        if let Some(dependencies) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
            for (dep_name, dep_value) in dependencies {
                if let Some(ignored_list) =
                    ignored_deps.rust_workspace_audit.ignore.get(&package.name)
                {
                    if ignored_list.contains(&dep_name) {
                        info!("ignoring {} dependency in '{}'", dep_name, package.name);
                        continue;
                    }
                }

                let is_workspace_dep = dep_value
                    .as_table()
                    .and_then(|t| t.get("workspace"))
                    .and_then(|w| w.as_bool())
                    .unwrap_or(false);

                if !is_workspace_dep {
                    error!(
                        "crate '{}' has non-workspace dependency: {}",
                        package.name, dep_name
                    );
                }
            }
        }
    }

    Ok(())
}

async fn load_ignored_dependencies(config_path: &str) -> Result<ConfigFile> {
    let content_str = match smol::fs::read_to_string(config_path).await {
        Ok(content) => content,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            warn!("Config file not found, continuing without ignored dependencies.");
            return Err(anyhow!("File {config_path} is not wound"));
        }
        Err(e) => return Err(anyhow!(e)),
    };

    Ok(toml::from_str(&content_str)?)
}
