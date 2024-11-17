use tauri::{webview_version, AppHandle};
use crate::AppState;
use crate::cli::{ShellClient, SystemShellClient, TauriShellClient};

pub async fn info_handler(app_state: &AppState, app_handle: &AppHandle){
    let shell_client = TauriShellClient {app_handle};
    println!("Environment");
    println!("\t- OS: {}", get_os_info(app_state).await);
    println!("\t- Webview: {}", get_webview_version().await);
    println!("\t- rustc: {}", get_rustc_version(&shell_client).await);
    println!("\t- cargo: {}", get_cargo_version(&shell_client).await);
    println!("\t- rustup: {}", get_rustup_version(&shell_client).await);
    println!("\t- Rust Toolchain: {}", get_rust_toolchain(&shell_client).await);
    println!("\t- node: {}", get_node_version(&shell_client).await);
    // FIXME: For some reason, it could not detect pnpm on my computer
    println!("\t- pnpm: {}", get_pnpm_version(&shell_client).await);

    println!("Packages");
    println!("\t- tauri [Rust]: {}", get_tauri_version().await);

    println!("App");
    println!("\t- Moss: {}", get_moss_version(app_handle).await);

}
async fn component_version(shell_client: &impl ShellClient, component: &str, arg: &str) -> String {
    let shell_output = shell_client.exec(
        component,
        vec![arg]
    ).await;

    match shell_output {
        Some(output) => {
            output.split('\n').next().unwrap_or("Not Found").to_string()
        }
        None => "Not Found".to_string()
    }
}

async fn get_os_info(app_state: &AppState) -> String {
    format!("{} {}", app_state.platform_info.os, app_state.platform_info.version)
}

async fn get_webview_version() -> String {
    format!("{}", webview_version().unwrap_or("Not Found".to_string()))
}

async fn get_rustc_version(shell_client: &impl ShellClient) -> String {
    component_version(shell_client, "rustc", "-V").await
}

async fn get_cargo_version(shell_client: &impl ShellClient) -> String {
    component_version(shell_client, "cargo", "-V").await
}

async fn get_rustup_version(shell_client: &impl ShellClient) -> String {
    component_version(shell_client, "rustup", "-V").await
}

async fn get_rust_toolchain(shell_client: &impl ShellClient) -> String {
    let shell_output = shell_client.exec(
        "rustup",
        vec!["show", "active-toolchain"]
    ).await;

    match shell_output {
        Some(output) => {
            output.split('\n').next().unwrap_or("Not Found").to_string()
        }
        None => "Not Found".to_string()
    }
}

async fn get_node_version(shell_client: &impl ShellClient) -> String {
    component_version(shell_client, "node", "-v").await
}

async fn get_pnpm_version(shell_client: &impl ShellClient) -> String {
    component_version(shell_client, "pnpm", "-v").await
}

async fn get_tauri_version() -> String {
    tauri::VERSION.to_string()
}

async fn get_moss_version(app_handle: &AppHandle) -> String {
    app_handle.config().clone().version.unwrap()
}


