// use platform_log::AnyLogger;

use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::{Arc, Mutex}};

use platform_log::log_service::{AnyLogger, BufferLogger, BufferableLogger, FileLogger, LogLevel};
use tauri::Emitter;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct WorkspaceLogService {
    file_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>,
    buffer_logger: Arc<Mutex<dyn BufferableLogger + Send + Sync + 'static>>,
    tao_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>,
}

impl WorkspaceLogService {
    pub fn new(
        file_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>, 
        buffer_logger: Arc<Mutex<dyn BufferableLogger + Send + Sync + 'static>>, 
        tao_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>
    ) -> Self {
        Self {
            file_logger,
            buffer_logger,
            tao_logger,
        }
    }

    // TODO: replace unwrap with match (in all functions)
    pub fn trace(&self, message: &str) {
        println!("trace!");
        if self.tao_logger.lock().unwrap().is_ready() {
            self.tao_logger.lock().unwrap().trace(message);
        } else {
            self.file_logger.lock().unwrap().trace(message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.tao_logger.lock().unwrap().is_ready() {
            self.tao_logger.lock().unwrap().debug(message);
        } else {
            self.file_logger.lock().unwrap().debug(message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.tao_logger.lock().unwrap().is_ready() {
            self.tao_logger.lock().unwrap().info(message);
        } else {
            self.file_logger.lock().unwrap().info(message);
        }
    }
    
    pub fn warning(&self, message: &str) {
        if self.tao_logger.lock().unwrap().is_ready() {
            self.tao_logger.lock().unwrap().warning(message);
        } else {
            self.file_logger.lock().unwrap().warning(message);
        }
    }
    
    pub fn error(&self, message: &str) {
        if self.tao_logger.lock().unwrap().is_ready() {
            self.tao_logger.lock().unwrap().error(message);
        } else {
            self.file_logger.lock().unwrap().error(message);
        }
    }
}

pub fn create_service() -> Result<WorkspaceLogService, std::io::Error> {
    let file_logger = FileLogger::new(
        PathBuf::from("/home/krail/projects/moss/logs/testlog-workspace.log"),
        8*1024*1024, // Max file size in bytes
    )?;

    let file_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(Mutex::new(file_logger));
    let buffer_logger: Arc<Mutex<dyn BufferableLogger + Send + Sync + 'static>> = Arc::new(Mutex::new(BufferLogger::new(5000))); // TODO: move this value to config/constant
    let tao_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(Mutex::new(TaoLogger::new()));
    
    Ok(WorkspaceLogService::new(file_logger, buffer_logger, tao_logger))
}

pub struct TaoLogger {
    level: LogLevel,
    formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>,
} 

impl TaoLogger {
    pub fn new() -> Self {
        return Self::default();
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
            formatter: Arc::new(|message, level| format!("[{:?}] {}", level, message)),
        }
    }
}

impl AnyLogger for TaoLogger {
    fn trace(&mut self, message: &str) {
        println!("trace in TaoLogger!");
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

    fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>) {
        self.formatter = formatter;
    }

    fn is_ready(&self) -> bool {
        // TODO: check if tracing has been initialized with the correct app_handler
        return true
    }
}