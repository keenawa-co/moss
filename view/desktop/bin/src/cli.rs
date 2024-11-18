mod cli_info;

use std::process::Command;
use tauri::{AppHandle, Manager};
use tauri_plugin_cli::SubcommandMatches;
use tauri_plugin_shell::ShellExt;

use crate::cli::cli_info::info_handler;
use crate::AppState;

pub trait ShellClient {
    async fn exec(&self, command: &str, args: Vec<&str>) -> Option<String>;
}

pub struct SystemShellClient;
impl ShellClient for SystemShellClient {
    async fn exec(&self, command: &str, args: Vec<&str>) -> Option<String> {
        let output = Command::new(command).args(args).output();
        match output {
            Ok(output) => Some(String::from_utf8_lossy(output.stdout.as_slice()).to_string()),
            Err(error) => None,
        }
    }
}
pub struct TauriShellClient<'a> {
    pub(crate) app_handle: &'a AppHandle,
}

impl ShellClient for TauriShellClient<'_> {
    async fn exec(&self, command: &str, args: Vec<&str>) -> Option<String> {
        let shell = self.app_handle.shell();
        let output = shell.command(command).args(args).output().await;
        match output {
            Ok(output) => Some(String::from_utf8_lossy(output.stdout.as_slice()).to_string()),
            Err(error) => None,
        }
    }
}

pub async fn cli_handler(subcommand: Box<SubcommandMatches>, app_handle: AppHandle) {
    let app_state = app_handle.state::<AppState>();

    match subcommand.name.as_str() {
        "info" => info_handler(&*app_state, &app_handle).await,
        _ => println!("Unknown subcommand"),
    }
}
