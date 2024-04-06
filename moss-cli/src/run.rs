use clap::Args;
use std::net::SocketAddr;

use crate::loader;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "The hostname and port to listen for connections on")]
    #[arg(default_value = "127.0.0.1:1355")]
    bind: SocketAddr,
    #[arg(default_value = "builtin/conf/preference.toml")]
    ubp_path: String,
}

pub async fn init(RunCmdArgs { bind, ubp_path }: RunCmdArgs) -> anyhow::Result<()> {
    let preference = loader::load_preference_file(ubp_path)?;
    let _ = moss_net::CONF.set(moss_net::Config { bind, preference });

    moss_net::bind().await.expect("Failed to start the server");
    return Ok(());
}
