use tauri::{webview_version, AppHandle};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_cli::{Matches, SubcommandMatches};
use std::process::Command;
use crate::AppState;

/// Sample matches:
/// Matches {
///      args: {},
///      subcommand: Some(SubcommandMatches {
///          name: "info",
///          matches: Matches {
///              args: {},
///              subcommand: None
///          }
///      })
/// }


fn rust_toolchain() -> String {
    let output = Command::new("rustup")
            .args(["show", "active-toolchain"])
            .output();
    match output {
        Ok(output) => {
            let output = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
            format!("Rust Toolchain: {}",
                    output.split('\n')
                        .next()
                        .unwrap_or("Not Found")
                        .to_string())
        }
        Err(e) => {
            "Rust Toolchain: Not Found".to_string()
        }
    }
}


fn component_version(component: &str, arg: &str) -> String {
    let output = Command::new(component)
            .args([arg])
            .output();
    match output {
        Ok(output) => {
            let output = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
            format!("{component}: {}",
                    output.split('\n')
                        .next()
                        .unwrap()
                        .trim_start_matches(&format!("{component} ")))
        }
        Err(e) => {
            format!("{}: Not Found", component)
        }
    }
}

pub fn cli_handler(matches: Matches, app_state: &AppState, app_handle: &AppHandle){
    let args = matches.args;
    let subcommand = matches.subcommand;
    if args.is_empty() && subcommand.is_none() {
        // The app is not running in CLI mode
        return;
    }
    match subcommand {
        Some(sub) => subcommand_handler(*sub, &app_state, app_handle),
        None => todo!()
    }
}

fn subcommand_handler(subcommand_matches: SubcommandMatches, app_state: &AppState, app_handle: &AppHandle) {
    // TODO: Other subcommands
    match subcommand_matches.name.as_str() {
        "info" => info_handler(app_state, app_handle),
        _ => println!("Unknown subcommand")
    }
}

fn info_handler(app_state: &AppState, app_handle: &AppHandle){
    println!("Environment");
    println!("\t- OS: {} {}", app_state.platform_info.os, app_state.platform_info.version);
    println!("\t- Webview: {}", webview_version().unwrap_or("unknown".to_string()));
    println!("\t- {}", component_version("rustc", "-V"));
    println!("\t- {}", component_version("cargo", "-V"));
    println!("\t- {}", component_version("rustup", "-V"));
    println!("\t- {}", rust_toolchain());
    println!("\t- {}", component_version("node", "-v"));
    // FIXME: For some reason, it could not detect pnpm on my computer
    println!("\t- {}", component_version("pnpm", "-v"));

    println!("Packages");
    println!("\t- tauri [Rust]: {}", tauri::VERSION);

    println!("App");
    println!("\t- Moss: {}", &app_handle.config().clone().version.unwrap());

    // TODO: How to correctly call an early exit?
    &app_handle.cleanup_before_exit();
    &app_handle.exit(0);
}

