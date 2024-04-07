use clap::Args;
use moss_net::Config;
use std::{net::SocketAddr, sync::Arc};
use surrealdb::{engine::remote::ws::Ws, Surreal};

use crate::loader;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "The hostname and port to listen for connections on")]
    #[arg(default_value = "127.0.0.1:1355")]
    bind: SocketAddr,
    #[arg(default_value = "builtin/conf/preference.toml")]
    preference_filepath: String,
    #[arg(default_value = "builtin/conf/configuration.toml")]
    configuration_filepath: String,
}

pub async fn init(
    RunCmdArgs {
        bind,
        preference_filepath,
        configuration_filepath,
    }: RunCmdArgs,
) -> anyhow::Result<()> {
    let configuration: crate::config::Config = loader::load_toml_file(configuration_filepath)?;

    let _ = moss_net::CONF.set(moss_net::Config {
        bind,
        preference: loader::load_toml_file(preference_filepath)?,
        surrealdb_client: Arc::new(Surreal::new::<Ws>(configuration.surrealdb.bind_addr()).await?),
    });

    moss_net::bind().await.expect("Failed to start the server");
    return Ok(());
}
