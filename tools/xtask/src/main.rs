mod metadata;
mod tasks;

use std::ptr::metadata;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tasks::rwa::{check_dependencies_job, RustWorkspaceAuditProvider};

use crate::metadata::load_cargo_metadata;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[macro_use]
extern crate anyhow;

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

    let rws_provider =
        RustWorkspaceAuditProvider::new(vec![Box::new(check_dependencies_job(args, metadata))]);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let metadata = load_cargo_metadata()?;

    match args.command {
        CliCommand::License(args) => tasks::license::run_license(args, metadata).await,
        CliCommand::Rwa(args) => tasks::rwa::check_dependencies_job(args, metadata).await,
    }
}
