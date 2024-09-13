use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use clap::Parser;
use toml::Value;

use tracing::{error, info, warn};

use cargo_metadata::Metadata;

#[derive(Parser)]
pub struct RwaArgs {}

pub fn run_rwa(_args: RwaArgs, workspace: Metadata) -> Result<()> {
    let ignored_deps = load_ignored_dependencies("config.toml")?;

    for package in workspace.workspace_packages() {
        info!("analyzing '{}'...", package.name);
        let cargo_toml_content = fs::read_to_string(&package.manifest_path)?;
        let cargo_toml: Value = toml::from_str(&cargo_toml_content)?;

        if let Some(dependencies) = cargo_toml.get("dependencies").and_then(|d| d.as_table()) {
            for (dep_name, dep_value) in dependencies {
                if let Some(ignored_list) = ignored_deps.get(&package.name) {
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

fn load_ignored_dependencies(config_path: &str) -> Result<HashMap<String, Vec<String>>> {
    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            warn!("config file not found, continue without ignored dependencies");
            return Ok(HashMap::new());
        }
        Err(e) => return Err(e.into()),
    };
    let config: Value = toml::from_str(&config_content)?;

    let mut ignored_deps = HashMap::new();
    if let Some(rwa_ignore) = config.get("rwa").and_then(|rwa| rwa.get("ignore")) {
        if let Some(table) = rwa_ignore.as_table() {
            for (crate_name, deps) in table {
                if let Some(deps_array) = deps.as_array() {
                    let deps_vec: Vec<String> = deps_array
                        .iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect();
                    ignored_deps.insert(crate_name.clone(), deps_vec);
                }
            }
        }
    }

    Ok(ignored_deps)
}
