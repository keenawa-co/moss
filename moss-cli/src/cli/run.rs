use clap::Args;

use sea_orm::{ActiveValue, Database, DeriveEntityModel, Set};
use sea_orm_migration::MigratorTrait;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, SqlitePool};
use std::{net::SocketAddr, path::Path, sync::Arc};
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tokio_util::sync::CancellationToken as TokioCancellationToken;

use crate::migration;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(
        long = "bind",
        help = "The hostname and port to listen for connections on",
        help_heading = "SocketAddr"
    )]
    bind: Option<SocketAddr>,
    #[arg(
        default_value = "conf/net_pref.toml",
        long = "net_pref_path",
        help_heading = "Path"
    )]
    net_pref_path: Box<Path>,
    #[arg(
        default_value = "conf/net_conf.toml",
        long = "net_conf_path",
        help_heading = "Path"
    )]
    net_conf_path: Box<Path>,
}

pub async fn init(
    RunCmdArgs {
        bind,
        net_pref_path: preference_filepath,
        net_conf_path,
    }: RunCmdArgs,
) -> anyhow::Result<()> {
    let conf: crate::config::Config = super::common::load_toml_file(net_conf_path)?;
    let conn = super::common::db_connection(Path::new("./moss.db")).await?;

    //  cancel_token is passed to all async functions requiring graceful termination
    let cancel_token = TokioCancellationToken::new();
    let _ = moss_net::CONF.set(moss_net::Config {
        bind: bind.unwrap_or(conf.net.endpoint_addr()),
        preference: super::common::load_toml_file(preference_filepath)?,
        conn: Arc::new(conn),
    });

    moss_net::bind(cancel_token).await?;

    Ok(())
}

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
