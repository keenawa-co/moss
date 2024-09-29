// use platform_log::AnyLogger;

use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::{Arc, Mutex}};

use platform_log::log_service::{AnyLogger, BufferLogger, BufferableLogger, FileLogger, LogLevel};
use tauri::Emitter;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use parking_lot::Mutex use instead of Mutex

pub struct WorkspaceLogService {
    file_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>,
    buffer_logger: Arc<parking_lot::Mutex<dyn BufferableLogger + Send + Sync + 'static>>,
    tao_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>,
}

impl WorkspaceLogService {
    pub fn new(
        file_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>, 
        buffer_logger: Arc<parking_lot::Mutex<dyn BufferableLogger + Send + Sync + 'static>>, 
        tao_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>
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
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().trace(message);
        } else {
            self.file_logger.lock().trace(message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().debug(message);
        } else {
            self.file_logger.lock().debug(message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().info(message);
        } else {
            self.file_logger.lock().info(message);
        }
    }
    
    pub fn warning(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().warning(message);
        } else {
            self.file_logger.lock().warning(message);
        }
    }
    
    pub fn error(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().error(message);
        } else {
            self.file_logger.lock().error(message);
        }
    }
}

pub fn create_service() -> Result<WorkspaceLogService, std::io::Error> {
    let file_logger = FileLogger::new(
        PathBuf::from("/home/krail/projects/moss/logs/testlog-workspace.log"),
        8*1024*1024, // Max file size in bytes
    )?;

    let file_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(parking_lot::Mutex::new(file_logger));
    let buffer_logger: Arc<parking_lot::Mutex<dyn BufferableLogger + Send + Sync + 'static>> = Arc::new(parking_lot::Mutex::new(BufferLogger::new(5000))); // TODO: move this value to config/constant
    let tao_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(parking_lot::Mutex::new(TaoLogger::new()));
    
    Ok(WorkspaceLogService::new(file_logger, buffer_logger, tao_logger))
}

pub struct TaoLogger {
    level: LogLevel,
} 

impl TaoLogger {
    pub fn new() -> Self {
        return Self::default();
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
        }
    }
}

impl AnyLogger for TaoLogger {
    fn trace(&mut self, message: &str) {
        if self.level >= LogLevel::Trace {
            trace!("{}", message);
        }
    }
    fn debug(&mut self, message: &str) {
        if self.level >= LogLevel::Debug {
            debug!("{}", message);
        }
    }
    fn info(&mut self, message: &str) {
        if self.level >= LogLevel::Info {
            info!("{}", message);
        }
    }
    fn warning(&mut self, message: &str) {
        if self.level >= LogLevel::Warning {
            warn!("{}", message);
        }
    }
    fn error(&mut self, message: &str) {
        if self.level >= LogLevel::Error {
            error!("{}", message);
        }
    }

    fn flush(&mut self) {
        unimplemented!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn is_ready(&self) -> bool {
        // TODO: check if tracing has been initialized with the correct app_handler
        return true
    }
}