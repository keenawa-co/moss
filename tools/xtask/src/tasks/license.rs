use smol::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use futures::future::join_all;
use tokio::{self};
use tracing::info;

use cargo_metadata::{Metadata, Package};

#[derive(Parser)]
pub struct LicenseArgs {}

pub async fn run_license(_args: LicenseArgs, workspace: Metadata) -> Result<()> {
    const LICENSE_FILES: &[&str] = &["LICENSE-MIT"];
    let default_license = workspace.workspace_root.join(LICENSE_FILES[0]);

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
                    default_license_clone.into(),
                    workspace_root_clone.into(),
                    LICENSE_FILES,
                )
                .await
            })
        })
        .collect::<Vec<_>>();

    for result in join_all(tasks).await {
        result.map_err(|err| anyhow!(err))??;
    }
    Ok(())
}

async fn handle_package_license(
    package: Package,
    default_license: PathBuf,
    workspace_root: PathBuf,
    license_files: &[&str],
) -> Result<()> {
    let crate_path = package
        .manifest_path
        .parent()
        .ok_or_else(|| anyhow!("no crate directory for '{}'", package.name))?;

    if let Some((symlink_path, license_index)) =
        get_first_license_symlink_path(crate_path, license_files).await
    {
        let root_license_path = pathdiff::diff_paths(
            workspace_root.join(license_files[license_index]),
            crate_path,
        )
        .ok_or_else(|| anyhow!("Failed to create relative path for root license"))?;

        if symlink_path.is_symlink() {
            let target = fs::read_link(&symlink_path).await?;
            if target != root_license_path {
                info!("updating symlink for '{}'", package.name);
                handle_update_symlink(&symlink_path, &root_license_path).await?;
            }
        } else {
            info!("replacing file with symlink for '{}'", package.name);
            handle_update_symlink(&symlink_path, &root_license_path).await?;
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
async fn handle_update_symlink(
    symlink_path: &PathBuf,
    root_license_path: &PathBuf,
) -> Result<(), anyhow::Error> {
    fs::remove_file(symlink_path).await?;
    create_symlink(root_license_path, symlink_path).await?;
    Ok(())
}

async fn get_first_license_symlink_path(
    crate_path: impl AsRef<Path>,
    license_files: &[&str],
) -> Option<(PathBuf, usize)> {
    let crate_path = crate_path.as_ref();
    for (index, &license_file) in license_files.iter().enumerate() {
        let path_to_license = crate_path.join(license_file);
        info!("analyzing '{}'...", path_to_license.display());
        match fs::symlink_metadata(&path_to_license).await {
            Ok(metadata) if metadata.is_file() || metadata.file_type().is_symlink() => {
                return Some((path_to_license, index));
            }
            _ => continue,
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
