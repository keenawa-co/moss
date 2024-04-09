use clap::Args;
use std::{net::SocketAddr, path::Path, sync::Arc};
use surrealdb::{engine::remote::ws::Ws, Surreal};
use tokio_util::sync::CancellationToken as TokioCancellationToken;

use crate::loader;

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
    let conf: crate::config::Config = loader::load_toml_file(net_conf_path)?;

    //  cancel_token is passed to all async functions requiring graceful termination
    let cancel_token = TokioCancellationToken::new();
    let surrealdb_client = Surreal::new::<Ws>(conf.surrealdb.endpoint_addr()).await?;
    surrealdb_client
        .use_ns(conf.surrealdb.endpoint.namespace)
        .use_db(conf.surrealdb.endpoint.database)
        .await?;

    let _ = moss_net::CONF.set(moss_net::Config {
        bind: bind.unwrap_or(conf.net.endpoint_addr()),
        preference: loader::load_toml_file(preference_filepath)?,
        surrealdb_client: Arc::new(surrealdb_client),
    });

    moss_net::bind(cancel_token).await?;

    Ok(())
}
