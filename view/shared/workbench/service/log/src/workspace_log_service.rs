// use platform_log::AnyLogger;

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use platform_log::log_service::{AnyLogger, BufferLogger, BufferableLogger, FileLogger, LogLevel};
use tauri::Emitter;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct WorkspaceLogService {
    file_logger: Rc<RefCell<dyn AnyLogger>>,
    buffer_logger: Rc<RefCell<dyn BufferableLogger>>,
    tao_logger: Rc<RefCell<dyn AnyLogger>>,
}

impl WorkspaceLogService {
    pub fn new(
        file_logger: Rc<RefCell<dyn AnyLogger>>, 
        buffer_logger: Rc<RefCell<dyn BufferableLogger>>, 
        tao_logger: Rc<RefCell<dyn AnyLogger>>
    ) -> Self {
        Self {
            file_logger,
            buffer_logger,
            tao_logger,
        }
    }

    pub fn trace(&self, message: &str) {
        self.tao_logger.borrow_mut().trace(message);
    }

    pub fn debug(&self, message: &str) {
        self.tao_logger.borrow_mut().debug(message);
    }

    pub fn info(&self, message: &str) {
        self.buffer_logger.borrow_mut().info(message);
    }
    
    pub fn warning(&self, message: &str) {
        self.tao_logger.borrow_mut().warning(message);
    }
    
    pub fn error(&self, message: &str) {
        self.tao_logger.borrow_mut().error(message);
    }
}

pub fn create_service(app_handle: tauri::AppHandle) -> Result<WorkspaceLogService, std::io::Error> {
    let file_logger = FileLogger::new(
        PathBuf::from("/home/krail/projects/moss/logs/testlog.log"),
        50, // Max file size in bytes
    )?;

    let file_logger: Rc<RefCell<dyn AnyLogger>> = Rc::new(RefCell::new(file_logger));
    let buffer_logger: Rc<RefCell<dyn BufferableLogger>> = Rc::new(RefCell::new(BufferLogger::new(5000))); // TODO: move this value to config/constant
    let tao_logger: Rc<RefCell<dyn AnyLogger>> = Rc::new(RefCell::new(TaoLogger::new(app_handle)));
    
    Ok(WorkspaceLogService::new(file_logger, buffer_logger, tao_logger))
}

pub struct TaoLogger {
    level: LogLevel,
    formatter: Box<dyn Fn(&str, LogLevel) -> String>,
} 

impl TaoLogger {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        init_custom_logging(app_handle); // todo: remove after init moves to lib.rs
        return Self::default();
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
            formatter: Box::new(|message, level| format!("[{:?}] {}", level, message)),
        }
    }
}

impl AnyLogger for TaoLogger {
    fn trace(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Trace) {
            trace!("{}", (self.formatter)(message, LogLevel::Trace));
        }
    }

    fn debug(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
            debug!("{}", (self.formatter)(message, LogLevel::Debug));
        }
    }

    fn info(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            info!("{}", (self.formatter)(message, LogLevel::Info));
        }
    }

    fn warning(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace | LogLevel::Warning ) {
            warn!("{}", (self.formatter)(message, LogLevel::Warning));
        }
    }

    fn error(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace | LogLevel::Warning | LogLevel::Error ) {
            error!("{}", (self.formatter)(message, LogLevel::Error));
        }
    }

    fn flush(&mut self) {
        unimplemented!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn set_format(&mut self, formatter: Box<dyn Fn(&str, LogLevel) -> String>) {
        self.formatter = formatter;
    }

    fn is_ready(&self) -> bool {
        unimplemented!()
    }
}


// TODO: move tracing init to lib.rs
fn init_custom_logging(app_handle: tauri::AppHandle) {
    struct TauriLogWriter {
        app_handle: tauri::AppHandle,
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

    // TODO: setup of logging level
    tracing_subscriber::registry()
    // log to frontend
        .with(
            tracing_subscriber::fmt::layer().with_writer(move || TauriLogWriter {
                app_handle: app_handle.clone(),
            }),
        )
        .init();
}