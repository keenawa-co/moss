use app::context::AsyncAppContext;
use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::migration::RootMigrator;

#[derive(Debug, Subcommand)]
pub enum MigrateCommandList {
    Up(MigrateUpCmdArgs),
}

#[derive(Args, Debug)]
pub struct MigrateUpCmdArgs {}

pub async fn cmd_migration_up(MigrateUpCmdArgs {}: MigrateUpCmdArgs) -> anyhow::Result<()> {
    seaorm_utl::conn::<RootMigrator>(&PathBuf::from("root.db")).await?;

    Ok(())
}