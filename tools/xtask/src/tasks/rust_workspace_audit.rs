use anyhow::Result;
use cargo_metadata::{Metadata, Package};
use clap::Parser;
use futures::future::join_all;
use smol::fs;
use std::sync::Arc;
use toml::Value;
use tracing::{error, trace};

use crate::config::ConfigFile;

#[derive(Parser)]
pub struct RustWorkspaceAuditCommandArgs {
    #[clap(long, default_value = "config.toml")]
    config_file_path: String,
}

pub async fn check_dependencies_job(
    args: RustWorkspaceAuditCommandArgs,
    metadata: Metadata,
) -> Result<()> {
    let config_file = Arc::new(ConfigFile::load(&args.config_file_path).await?);
    let tasks = metadata
        .packages
        .into_iter()
        .filter(|p| metadata.workspace_members.contains(&p.id))
        .map(|package| {
            let config_file_clone = Arc::clone(&config_file);

            tokio::task::spawn(async move {
                trace!("analyzing '{}'...", package.name);
                let cargo_toml_content = match fs::read_to_string(&package.manifest_path).await {
                    Ok(content) => content,
                    Err(e) => {
                        return Err(anyhow!(
                            "Failed to read manifest file for {}: {}",
                            package.manifest_path,
                            e
                        ));
                    }
                };

                let cargo_toml = match toml::from_str(&cargo_toml_content) {
                    Ok(value) => value,
                    Err(e) => {
                        return Err(anyhow!(
                            "Failed to parse TOML file for {}: {}",
                            package.manifest_path,
                            e
                        ));
                    }
                };

                handle_package_dependencies(cargo_toml, &config_file_clone, package).await;

                Ok(())
            })
        })
        .collect::<Vec<_>>();

    for result in join_all(tasks).await {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => error!("Error processing package: {}", e),
            Err(e) => error!("Task panicked: {}", e),
        }
    }

    Ok(())
}

async fn handle_package_dependencies(
    cargo_toml: Value,
    ignored_deps: &ConfigFile,
    package: Package,
) {
    if let Some(dependencies) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
        for (dep_name, dep_value) in dependencies {
            if let Some(ignored_list) = ignored_deps.rust_workspace_audit.ignore.get(&package.name)
            {
                if ignored_list.contains(&dep_name) {
                    trace!("ignoring {} dependency in '{}'", dep_name, package.name);
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
