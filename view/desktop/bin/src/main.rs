// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use desktop_app_lib::constants::{RUNTIME_MAX_BLOCKING_THREADS, RUNTIME_STACK_SIZE};

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(*RUNTIME_MAX_BLOCKING_THREADS)
        .thread_stack_size(*RUNTIME_STACK_SIZE)
        .build()
        .unwrap()
        .block_on(async {
            tauri::async_runtime::set(tokio::runtime::Handle::current());
            desktop_app_lib::run();
        })
}
