mod tasks;
mod workspace;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cargo xtask")]
struct Args {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    License(tasks::license::LicenseArgs),
    Rwa(tasks::rwa::RwaArgs),
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        CliCommand::License(args) => tasks::license::run_license(args),
        CliCommand::Rwa(args) => tasks::rwa::run_rwa(args),
    }
}
