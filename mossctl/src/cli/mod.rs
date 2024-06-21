mod docs;
mod migrate;
mod run;
mod utl;

use anyhow::Context;
use app::{
    context::{AppCell, AppContext, AsyncAppContext},
    context_compact::AppContextCompact,
};
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

pub fn init(ctx: &mut AppContextCompact) -> ExitCode {
    let args = CLI::parse();

    let output = match args.command {
        Commands::Run(args) => ctx
            .block_on(move |ctx: &AppContextCompact| async move { run::cmd_run(&ctx, args).await }),
        Commands::Migrate(cmd) => match cmd {
            MigrateCommandList::Up(args) => {
                ctx.block_on(move |_| async move { migrate::cmd_migration_up(args).await })
            }
        },
        Commands::Docs(cmd) => match cmd {
            DocsCommandList::Schema(args) => {
                ctx.block_on(move |_| async move { docs::cmd_graphql_schema(args).await })
            }
        },
    };

    if let Err(e) = output {
        error!("{}", e);
        return ExitCode::FAILURE;
    } else {
        return ExitCode::SUCCESS;
    }
}
