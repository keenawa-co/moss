pub mod cli_info;

use clap::{Parser, Subcommand};
use std::process::Command;
use std::sync::LazyLock;
use tauri::{AppHandle, Config, Manager};

use crate::cli::cli_info::info_handler;
use crate::AppState;

pub(crate) static APP_CONFIG: LazyLock<Config> =
    LazyLock::new(|| serde_json::from_str(include_str!("../tauri.conf.json")).unwrap_or_default());
#[derive(Parser, Debug)]
#[clap(version)]
pub struct MossArgs {
    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
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
    let args = MossArgs::parse();
    let command = args.command.unwrap();
    match command {
        CliCommand::Info => {
            cli_info::info_handler().await;
        }
    }
}
