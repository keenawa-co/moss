use clap::Args;
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;

use crate::migration;

#[derive(Args, Debug)]
pub struct MigrateCmdArgs {}

pub async fn init(MigrateCmdArgs {}: MigrateCmdArgs) -> anyhow::Result<()> {
    let conn = super::utl::db_connection(&PathBuf::from("./")).await?;
    migration::Migrator::up(&conn, None).await?;

    Ok(())
}
