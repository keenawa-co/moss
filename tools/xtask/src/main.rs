mod tasks;
mod workspace;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::workspace::load_workspace;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "cargo xtask")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    License(tasks::license::LicenseArgs),
    Rwa(tasks::rwa::WorkspaceAuditCommandArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let workspace = load_workspace()?;

    match args.command {
        CliCommand::License(args) => tasks::license::run_license(args, workspace).await,
        CliCommand::Rwa(args) => tasks::rwa::check_dependencies_job(args, workspace).await,
    }
}
