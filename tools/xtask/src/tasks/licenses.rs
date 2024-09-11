use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;

use crate::workspace::load_workspace;

#[derive(Parser)]
pub struct LicensesArgs {}

pub fn run_licenses(_args: LicensesArgs) -> Result<()> {
    const LICENSE_FILES: &[&str] = &["LICENSE-MIT"];

    let workspace = load_workspace()?;
    let root_license = workspace.workspace_root.join(LICENSE_FILES[0]);

    for package in workspace.workspace_packages() {
        let crate_dir = package
            .manifest_path
            .parent()
            .ok_or_else(|| anyhow!("no crate directory for {}", package.name))?;

        if let Some(license_file) = first_license_file(crate_dir, LICENSE_FILES) {
            if license_file.is_symlink() {
                let target = fs::read_link(&license_file)?;
                if target != root_license {
                    println!("updating symlink for {}", package.name);
                    fs::remove_file(&license_file)?;
                    create_symlink(&root_license.as_std_path(), &license_file)?;
                }
            } else {
                println!("replacing file with symlink for {}", package.name);
                fs::remove_file(&license_file)?;
                create_symlink(&root_license.as_std_path(), &license_file)?;
            }
        } else {
            println!("creating license symlink for {}", package.name);
            let license_new_path = crate_dir.join(LICENSE_FILES[0]);
            create_symlink(&root_license.as_std_path(), &license_new_path.as_std_path())?;
        }
    }

    Ok(())
}

fn first_license_file(path: impl AsRef<Path>, license_files: &[&str]) -> Option<PathBuf> {
    for license_file in license_files {
        let path_to_license = path.as_ref().join(license_file);
        if path_to_license.exists() {
            return Some(path_to_license);
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
