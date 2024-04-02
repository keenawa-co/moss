use clap::Args;
use std::net::SocketAddr;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "The hostname and port to listen for connections on")]
    #[arg(default_value = "127.0.0.1:3000")]
    bind: SocketAddr,
}

pub async fn init(RunCmdArgs { bind }: RunCmdArgs) -> anyhow::Result<()> {
    let _ = server::CONF.set(server::Config { bind });

    server::init().await.expect("Failed to start the server");
    return Ok(());
}
