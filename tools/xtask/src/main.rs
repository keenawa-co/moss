mod config;
mod metadata;
mod tasks;

use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use tasks::TaskRunner;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::metadata::load_cargo_metadata;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate tracing;

#[derive(Parser)]
#[command(name = "cargo xtask")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    License(tasks::license::LicenseCommandArgs),
    Rwa(tasks::rust_workspace_audit::RustWorkspaceAuditCommandArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    let metadata = load_cargo_metadata()?;
    let mut runner = TaskRunner::new();

    match args.command {
        CliCommand::License(args) => {
            runner.spawn_job(tasks::license::run_license(args, metadata));
            runner.run().await
        }
        CliCommand::Rwa(args) => {
            runner.spawn_job(tasks::rust_workspace_audit::check_dependencies_job(
                args, metadata,
            ));
            runner.run().await
        }
    }
}
