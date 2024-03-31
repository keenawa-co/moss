use clap::Args;

#[derive(Args, Debug)]
pub struct RunCmdArgs {
    #[arg(help = "UI host port")]
    port: usize,
}

pub async fn init(RunCmdArgs { port }: RunCmdArgs) -> anyhow::Result<()> {
    println!("Hello, world! {}", port);

    Ok(())
}
