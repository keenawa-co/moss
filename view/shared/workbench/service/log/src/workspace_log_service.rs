// use platform_log::AnyLogger;

use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::{Arc, Mutex}};

use platform_log::log_service::{AnyLogger, BufferLogger, BufferableLogger, FileLogger, LogLevel, LogService};
use tauri::Emitter;
use tracing::{debug, error, event, info, trace, warn, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use parking_lot::Mutex use instead of Mutex

pub struct WorkspaceLogService {
    // platformService: LogService,
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
            self.tao_logger.lock().log(LogLevel::Trace, message);
        } else {
            self.file_logger.lock().log(LogLevel::Trace, message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().log(LogLevel::Debug, message);;
        } else {
            self.file_logger.lock().log(LogLevel::Debug, message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().log(LogLevel::Info, message);
        } else {
            self.file_logger.lock().log(LogLevel::Info, message);
        }
    }
    
    pub fn warning(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().log(LogLevel::Warning, message);
        } else {
            self.file_logger.lock().log(LogLevel::Warning, message);
        }
    }
    
    pub fn error(&self, message: &str) {
        if self.tao_logger.lock().is_ready() {
            self.tao_logger.lock().log(LogLevel::Error, message);
        } else {
            self.file_logger.lock().log(LogLevel::Error, message);
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
    fn log(&mut self, level: LogLevel, message: &str) {
        if self.level < level {
            // do not log
            return
        }

        match level {
            LogLevel::Off => return, // ignore input
            LogLevel::Trace => event!(Level::TRACE, tao=true, message),
            LogLevel::Debug => event!(Level::DEBUG, tao=true, message),
            LogLevel::Info => event!(Level::INFO, tao=true, message),
            LogLevel::Warning => event!(Level::WARN, tao=true, message),
            LogLevel::Error => event!(Level::ERROR, tao=true, message),
        }
    }
    // fn trace(&mut self, message: &str) {
    //     if self.level >= LogLevel::Trace {
    //         trace!("{}", message);
    //     }
    // }
    // fn debug(&mut self, message: &str) {
    //     if self.level >= LogLevel::Debug {
    //         debug!("{}", message);
    //     }
    // }
    // fn info(&mut self, message: &str) {
    //     if self.level >= LogLevel::Info {
    //         info!("{}", message);
    //     }
    // }
    // fn warning(&mut self, message: &str) {
    //     if self.level >= LogLevel::Warning {
    //         warn!("{}", message);
    //     }
    // }
    // fn error(&mut self, message: &str) {
    //     if self.level >= LogLevel::Error {
    //         error!("{}", message);
    //     }
    // }

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