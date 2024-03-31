mod run;

use self::run::RunCmdArgs;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(bin_name = "app")]
#[command(about = "App command-line interface and server")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run the application server")]
    Run(RunCmdArgs),
}

pub async fn init() -> ExitCode {
    let args = CLI::parse();

    let output = match args.command {
        Commands::Run(args) => run::init(args).await,
    };

    if let Err(e) = output {
        println!("{}", e);
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}
