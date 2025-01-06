use anyhow::Result;
use cargo_metadata::{Metadata, Package};
use clap::Parser;
use smol::fs;
use std::sync::Arc;
use tokio::task::JoinSet;
use toml::Value;
use tracing::{error, trace};

use crate::config::{ConfigFile, RustWorkspaceAuditConfig};

#[derive(Parser)]
pub struct RustWorkspaceAuditCommandArgs {
    #[clap(long, default_value = "config.toml")]
    config_file_path: String,
}

pub async fn check_dependencies_job(
    args: RustWorkspaceAuditCommandArgs,
    metadata: Metadata,
    fail_fast: bool,
) -> Result<()> {
    let mut task_set = JoinSet::new();

    let config_file = Arc::new(ConfigFile::load(&args.config_file_path).await?);
    metadata
        .packages
        .into_iter()
        .filter(|p| metadata.workspace_members.contains(&p.id))
        .for_each(|package| {
            let config_file_clone = Arc::clone(&config_file);

            task_set.spawn(async move {
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

                match handle_package_dependencies(
                    cargo_toml,
                    &config_file_clone.rust_workspace_audit,
                    package,
                )
                .await
                {
                    Ok(()) => Ok(()),
                    Err(e) => Err(e),
                }
            });
        });

    let mut failure_count = 0;
    while let Some(result) = task_set.join_next().await {
        match result {
            Ok(Ok(())) => {}
            Ok(Err(_)) => {
                failure_count += 1;
                if fail_fast {
                    task_set.abort_all();
                    return Err(anyhow!("Failing Fast"));
                }
            }
            Err(_) => {
                failure_count += 1;
                if fail_fast {
                    task_set.abort_all();
                    return Err(anyhow!("Failing Fast"));
                }
            }
        }
    }
    if failure_count > 0 {
        Err(anyhow!("{} Checks Failed", failure_count))
    } else {
        info!("All Checks Passed");
        Ok(())
    }
}

async fn handle_package_dependencies(
    cargo_toml: Value,
    rwa_config: &RustWorkspaceAuditConfig,
    package: Package,
) -> Result<()> {
    if let Some(dependencies) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
        for (dep_name, dep_value) in dependencies {
            if rwa_config.global_ignore.contains(dep_name) {
                trace!("ignoring {} dependency in '{}'", dep_name, package.name);
                continue;
            }
            if let Some(ignored_list) = rwa_config.crate_ignore.get(&package.name) {
                if ignored_list.contains(dep_name) {
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
                return Err(anyhow!(
                    "crate '{}' has non-workspace dependency: {}",
                    package.name,
                    dep_name
                ));
            }
        }
    }
    Ok(())
}
