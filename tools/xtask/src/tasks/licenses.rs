use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::workspace::load_workspace;

#[derive(Parser)]
pub struct LicensesArgs {}

pub fn run_licenses(_args: LicensesArgs) -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    const LICENSE_FILES: &[&str] = &["LICENSE-MIT"];

    let workspace = load_workspace()?;
    let default_license = workspace.workspace_root.join(LICENSE_FILES[0]);

    for package in workspace.workspace_packages() {
        let crate_path = package
            .manifest_path
            .parent()
            .ok_or_else(|| anyhow!("no crate directory for '{}'", package.name))?;

        if let Some((symlink_path, license_index)) =
            get_first_license_symlink_path(crate_path, LICENSE_FILES)
        {
            let root_license_path = pathdiff::diff_paths(
                workspace.workspace_root.join(LICENSE_FILES[license_index]),
                crate_path,
            )
            .expect("failed to create relative path for root license");
            if symlink_path.is_symlink() {
                let target = fs::read_link(&symlink_path)?;
                if target != root_license_path {
                    info!("updating symlink for '{}'", package.name);
                    fs::remove_file(&symlink_path)?;
                    create_symlink(&root_license_path, &symlink_path)?;
                }
            } else {
                info!("replacing file with symlink for '{}'", package.name);
                fs::remove_file(&symlink_path)?;
                create_symlink(&root_license_path, &symlink_path)?;
            }
        } else {
            info!("creating license symlink for '{}'", package.name);
            let new_symlink_path = crate_path.join(LICENSE_FILES[0]);
            let default_license_path = pathdiff::diff_paths(&default_license, crate_path)
                .ok_or_else(|| anyhow!("failed to create relative path for default license"))?;
            create_symlink(&default_license_path, &new_symlink_path.as_std_path())?;
        }
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
fn create_symlink(license_path: &Path, symlink_path: &Path) -> Result<()> {
    std::os::unix::fs::symlink(license_path, symlink_path)?;
    Ok(())
}

#[cfg(windows)]
fn create_symlink(license_path: &Path, symlink_path: &Path) -> Result<()> {
    std::os::windows::fs::symlink_file(license_path, symlink_path)?;
    Ok(())
}
