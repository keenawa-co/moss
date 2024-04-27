mod docs;
mod migrate;
mod run;
mod utl;

use clap::{Parser, Subcommand};
use common::APP_NAME;
use std::process::ExitCode;

use self::{docs::DocsCommandList, migrate::MigrateCmdArgs, run::RunCmdArgs};
#[derive(Parser, Debug)]
#[command(name = APP_NAME, bin_name = APP_NAME)]
#[command(about = "Moss command-line interface and server")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run the application server")]
    Run(RunCmdArgs),

    #[command(about = "...")]
    Migrate(MigrateCmdArgs),

    #[command(subcommand)]
    Docs(DocsCommandList),
}

pub async fn init() -> ExitCode {
    let args = CLI::parse();

    let output = match args.command {
        Commands::Run(args) => run::init(args).await,
        Commands::Migrate(args) => migrate::init(args).await,
        Commands::Docs(cmd) => match cmd {
            DocsCommandList::Schema(args) => docs::new(args).await,
        },
    };

    if let Err(e) = output {
        println!("{}", e);
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}
