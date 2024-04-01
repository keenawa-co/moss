use clap::Args;
use server::Server;

use std::net::Ipv4Addr;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "UI host port")]
    port: u16,
}

pub async fn init(RunCmdArgs { port }: RunCmdArgs) -> anyhow::Result<()> {
    let s = Server::new(Ipv4Addr::new(127, 0, 0, 1), port);
    s.serve().await.expect("Failed to start the server");

    return Ok(());
}
