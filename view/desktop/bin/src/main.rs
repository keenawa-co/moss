use clap::Parser;
use desktop_app_lib::cli;
use desktop_app_lib::cli::MossArgs;
use desktop_app_lib::constants::{RUNTIME_MAX_BLOCKING_THREADS, RUNTIME_STACK_SIZE};

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(*RUNTIME_MAX_BLOCKING_THREADS)
        .thread_stack_size(*RUNTIME_STACK_SIZE)
        .build()
        .unwrap()
        .block_on(async {
            if std::env::args().len() > 1 {
                cli::cli_handler().await;
            } else {
                // TODO: Find an elegant alternative to prevent console window for GUI
                #[cfg(all(target_os = "windows", not(debug_assertions)))]
                {
                    unsafe {
                        // Detaching console on Windows for release build
                        winapi::um::wincon::FreeConsole();
                    }
                }
                tauri::async_runtime::set(tokio::runtime::Handle::current());
                desktop_app_lib::run();
            }
        })
}
