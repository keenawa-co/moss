// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::error;

fn main() {
    if let Err(err) = app_lib::run() {
        error!("{err:#?}")
    }
}
