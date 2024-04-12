use clap::Args;
use sea_orm_migration::MigratorTrait;
use std::path::Path;

use crate::migration;

#[derive(Args, Debug)]
pub struct MigrateCmdArgs {}

pub async fn init(MigrateCmdArgs {}: MigrateCmdArgs) -> anyhow::Result<()> {
    let conn = super::common::db_connection(Path::new("./moss.db")).await?;
    migration::Migrator::up(&conn, None).await?;

    Ok(())
}
