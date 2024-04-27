use clap::{Args, Subcommand};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;

use crate::migration;

#[derive(Debug, Subcommand)]
pub enum MigrateCommandList {
    Up(MigrateUpCmdArgs),
}

#[derive(Args, Debug)]
pub struct MigrateUpCmdArgs {}

pub async fn cmd_migration_up(MigrateUpCmdArgs {}: MigrateUpCmdArgs) -> anyhow::Result<()> {
    let conn = super::utl::db_connection(&PathBuf::from("./")).await?;
    migration::Migrator::up(&conn, None).await?;

    Ok(())
}
