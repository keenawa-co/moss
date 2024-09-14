use smol::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use tokio::{self, task::JoinHandle};

use tracing::info;

use cargo_metadata::{Metadata, Package};

#[derive(Parser)]
pub struct LicenseArgs {}

pub async fn run_license(_args: LicenseArgs, workspace: Metadata) -> Result<()> {
    const LICENSE_FILES: &[&str] = &["LICENSE-MIT"];
    let default_license = workspace.workspace_root.join(LICENSE_FILES[0]);

    //let mut tasks = Vec::new();

    //let workspace_root_clone = workspace.workspace_root.clone();

    let tasks = workspace
        .packages
        .into_iter()
        .filter(|p| workspace.workspace_members.contains(&p.id))
        .map(|package| {
            let workspace_root_clone = workspace.workspace_root.clone();
            let default_license_clone = default_license.clone();
            tokio::task::spawn(async move {
                handle_package_license(
                    package,
                    default_license_clone.as_std_path(),
                    workspace_root_clone.as_std_path(),
                    LICENSE_FILES,
                )
            })
        })
        .collect::<Vec<_>>();

    /*
    for package in workspace.workspace_packages() {
        let default_license = default_license.clone();
        let workspace_root = workspace.workspace_root.clone();

        tasks.push(tokio::task::spawn(async move {
            /*
            handle_package_license(
                package,
                default_license.as_std_path(),
                workspace_root.as_std_path(),
                LICENSE_FILES,
            )
            .await
            */
        }));
    }

    /*
    for task in tasks {
        task.await??;
    }
    */
    */

    Ok(())
}
async fn handle_package_license(
    package: Package,
    default_license: &Path,
    workspace_root: &Path,
    license_files: &[&str],
) -> Result<()> {
    let crate_path = package
        .manifest_path
        .parent()
        .ok_or_else(|| anyhow!("no crate directory for '{}'", package.name))?;

    if let Some((symlink_path, license_index)) =
        get_first_license_symlink_path(crate_path, license_files)
    {
        let root_license_path = pathdiff::diff_paths(
            workspace_root.join(license_files[license_index]),
            crate_path,
        )
        .expect("failed to create relative path for root license");

        if symlink_path.is_symlink() {
            let target = fs::read_link(&symlink_path).await?;
            if target != root_license_path {
                info!("updating symlink for '{}'", package.name);
                fs::remove_file(&symlink_path).await?;
                create_symlink(&root_license_path, &symlink_path).await?;
            }
        } else {
            info!("replacing file with symlink for '{}'", package.name);
            fs::remove_file(&symlink_path).await?;
            create_symlink(&root_license_path, &symlink_path).await?;
        }
    } else {
        info!("creating license symlink for '{}'", package.name);
        let new_symlink_path = crate_path.join(license_files[0]);
        let default_license_path = pathdiff::diff_paths(default_license, crate_path)
            .ok_or_else(|| anyhow!("failed to create relative path for default license"))?;
        create_symlink(&default_license_path, &new_symlink_path.as_std_path()).await?;
    }

    Ok(())
}

fn get_first_license_symlink_path(
    crate_path: impl AsRef<Path>,
    license_files: &[&str],
) -> Option<(PathBuf, usize)> {
    for (index, license_file) in license_files.iter().enumerate() {
        let path_to_license = crate_path.as_ref().join(license_file);
        info!("analyzing '{}'...", path_to_license.display());
        if path_to_license.exists() || std::fs::read_link(&path_to_license).is_ok() {
            return Some((path_to_license, index));
        }
    }
    None
}

#[cfg(unix)]
async fn create_symlink(license_path: &Path, symlink_path: &Path) -> Result<()> {
    smol::fs::unix::symlink(license_path, symlink_path).await?;
    Ok(())
}

#[cfg(windows)]
async fn create_symlink(license_path: &Path, symlink_path: &Path) -> Result<()> {
    smol::fs::windows::symlink_file(license_path, symlink_path).await?;
    Ok(())
}
