// use platform_log::AnyLogger;

use platform_log::log_service::{AnyLogger, LogLevel};

pub struct WorkspaceLogService {
    file_logger: Box<dyn AnyLogger>,
    buffer_logger: Box<dyn AnyLogger>,
    tao_logger: Box<dyn AnyLogger>,
}

impl WorkspaceLogService {
    pub fn new(
        file_logger: Box<dyn AnyLogger>, 
        buffer_logger: Box<dyn AnyLogger>, 
        tao_logger: Box<dyn AnyLogger>
    ) -> Self {
        Self {
            file_logger,
            buffer_logger,
            tao_logger,
        }
    }
}

// fn init_custom_logging(app_handle: tauri::AppHandle) {
//     struct TauriLogWriter {
//         app_handle: tauri::AppHandle,
//     }

//     impl std::io::Write for TauriLogWriter {
//         fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//             let log_message = String::from_utf8_lossy(buf).to_string();
//             let _ = self.app_handle.emit("logs-stream", log_message);
//             Ok(buf.len())
//         }

//         fn flush(&mut self) -> std::io::Result<()> {
//             Ok(())
//         }
//     }

//     tracing_subscriber::registry()
//         // log to stdout
//         .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
//         // log to frontend
//         .with(
//             tracing_subscriber::fmt::layer().with_writer(move || TauriLogWriter {
//                 app_handle: app_handle.clone(),
//             }),
//         )
//         .init();

//     event!(tracing::Level::DEBUG, "Logging init");
//     info!("Logging initialized");
// }


pub struct TaoLogger {
    level: LogLevel,
    formatter: Box<dyn Fn(&str, LogLevel) -> String>,
    app_handle: tauri::AppHandle,
} 


impl TaoLogger {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self::default(app_handle)
    }

    fn default(app_handle: tauri::AppHandle) -> Self {
        Self {
            level: LogLevel::Trace,
            formatter: Box::new(|message, level| format!("[{:?}] {}", level, message)),
            app_handle: app_handle
        }
    }

    fn init_tracing() {
        tracing_subscriber::registry()
        // log to frontend
            .with(
                tracing_subscriber::fmt::layer().with_writer(move || TauriLogWriter {
                    app_handle: app_handle.clone(),
                }),
            )
            .init();

    }
}

    impl std::io::Write for TauriLogWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let log_message = String::from_utf8_lossy(buf).to_string();
            let _ = self.app_handle.emit("logs-stream", log_message);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }


impl AnyLogger for TaoLogger {
    fn trace(&mut self, message: &str) {
        todo!()
    }

    fn debug(&mut self, message: &str) {
        todo!()
    }

    fn info(&mut self, message: &str) {
        todo!()
    }

    fn warning(&mut self, message: &str) {
        todo!()
    }

    fn error(&mut self, message: &str) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }

    fn set_level(&mut self, level: platform_log::log_service::LogLevel) {
        todo!()
    }

    fn set_format(&mut self, formatter: Box<dyn Fn(&str, platform_log::log_service::LogLevel) -> String>) {
        todo!()
    }

    fn is_ready(&self) -> bool {
        todo!()
    }
}