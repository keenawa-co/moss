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
        let crate_dir = package
            .manifest_path
            .parent()
            .ok_or_else(|| anyhow!("no crate directory for '{}'", package.name))?;

        if let Some((license_file, license_index)) = first_license_file(crate_dir, LICENSE_FILES) {
            let root_license = pathdiff::diff_paths(
                workspace.workspace_root.join(LICENSE_FILES[license_index]),
                crate_dir,
            )
            .expect("failed to create relative path for root license");
            if license_file.is_symlink() {
                let target = fs::read_link(&license_file)?;
                if target != root_license {
                    info!("updating symlink for '{}'", package.name);
                    fs::remove_file(&license_file)?;
                    create_symlink(&root_license, &license_file)?;
                }
            } else {
                info!("replacing file with symlink for '{}'", package.name);
                fs::remove_file(&license_file)?;
                create_symlink(&root_license, &license_file)?;
            }
        } else {
            info!("creating license symlink for '{}'", package.name);
            let license_new_path = crate_dir.join(LICENSE_FILES[0]);
            let relative_default_license = pathdiff::diff_paths(&default_license, crate_dir)
                .ok_or_else(|| anyhow!("failed to create relative path for default license"))?;
            create_symlink(&relative_default_license, &license_new_path.as_std_path())?;
        }
    }

    Ok(())
}

fn first_license_file(path: impl AsRef<Path>, license_files: &[&str]) -> Option<(PathBuf, usize)> {
    for (index, license_file) in license_files.iter().enumerate() {
        let path_to_license = path.as_ref().join(license_file);
        info!("analyzing '{}'...", path_to_license.display());
        if std::fs::read_link(&path_to_license).is_ok() {
            return Some((path_to_license, index));
        }
    }

    None
}

#[cfg(unix)]
fn create_symlink(src: &Path, dst: &Path) -> Result<()> {
    std::os::unix::fs::symlink(src, dst)?;
    Ok(())
}

#[cfg(windows)]
fn create_symlink(src: &Path, dst: &Path) -> Result<()> {
    std::os::windows::fs::symlink_file(src, dst)?;
    Ok(())
}
