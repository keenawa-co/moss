mod docs;
mod migrate;
mod run;
mod utl;

use anyhow::Context;
use app::context::{AppCell, AppContext, AsyncAppContext};
use clap::{Parser, Subcommand};
use std::{process::ExitCode, sync::Arc};

use self::{docs::DocsCommandList, migrate::MigrateCommandList, run::RunCmdArgs};

#[derive(Parser, Debug)]
#[command(name = "moss", bin_name = "moss")]
#[command(about = "Moss command-line interface and server")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

/// Defines the available commands for the application.
/// Each command can have its own options and subcommands, allowing for complex operations
/// like running the server, performing database migrations, or generating documentation.
#[derive(Debug, Subcommand)]
enum Commands {
    /// Run the application server.
    /// This command starts the server and listens for incoming connections on a specified port.
    /// It requires network configuration, such as the IP address and port number.
    #[command(about = "Run the application server")]
    Run(RunCmdArgs),

    /// Perform database migrations.
    /// This set of subcommands allows for applying, reverting, and managing database schema changes.
    /// It is crucial for maintaining the database state consistent with the application's models.
    #[command(subcommand)]
    Migrate(MigrateCommandList),

    /// Generate and manage documentation.
    /// These subcommands help in creating, updating, and organizing the documentation of the
    /// GraphQL schema and other internal documentation needs of the application.
    #[command(subcommand)]
    Docs(DocsCommandList),
}

pub fn init(ctx: &AppContext) -> ExitCode {
    let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
    else {
        panic!("failed to build async runtime");
    };

    let args = CLI::parse();

    let output = match args.command {
        Commands::Run(args) => runtime.block_on(run::cmd_run(&ctx, args)),
        Commands::Migrate(cmd) => match cmd {
            MigrateCommandList::Up(args) => runtime.block_on(migrate::cmd_migration_up(args)),
        },
        Commands::Docs(cmd) => match cmd {
            DocsCommandList::Schema(args) => runtime.block_on(docs::cmd_graphql_schema(args)),
        },
    };

    if let Err(e) = output {
        error!("{}", e);
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}
