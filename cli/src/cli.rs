pub mod cli_info;

use crate::cli::cli_info::info_handler;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display various environment information about the moss app
    Info,
}

pub trait ShellClient {
    async fn exec(&self, command: &str, args: Vec<&str>) -> Option<String>;
}

pub struct SystemShellClient;
impl ShellClient for SystemShellClient {
    async fn exec(&self, command: &str, args: Vec<&str>) -> Option<String> {
        let output = Command::new(command).args(args).output();
        match output {
            Ok(output) => Some(String::from_utf8_lossy(output.stdout.as_slice()).to_string()),
            Err(_) => None,
        }
    }
}

pub async fn cli_handler() {
    let args = Cli::parse();
    match args.command {
        Commands::Info => {
            info_handler().await;
        }
    }
}
