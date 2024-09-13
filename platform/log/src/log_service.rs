use std::{collections::VecDeque, default, rc::{Rc, Weak}};

pub struct LogService {
    file_logger: Rc<dyn AnyLogger>,
    buffer_logger: BufferLogger,
    cli_logger: Rc<dyn AnyLogger>,
}

impl LogService {
    pub fn new(file_logger: Rc<dyn AnyLogger>, buffer_logger: Rc<dyn AnyLogger>, cli_logger: Rc<dyn AnyLogger>) -> Self {
        Self {
            file_logger,
            buffer_logger,
            cli_logger,
        }
    }

    pub fn trace(&self, message: &str) {
        self.buffer_logger.trace(message);
    }

    pub fn debug(&self, message: &str) {
        self.buffer_logger.debug(message);
    }

    pub fn info(&self, message: &str) {
        self.buffer_logger.info(message);
    }
    
    pub fn warning(&self, message: &str) {
        self.buffer_logger.warning(message);
    }
    
    pub fn error(&self, message: &str) {
        self.buffer_logger.error(message);
    }

    pub fn flush_buffer_logger_to_cli(&mut self) {
        self.buffer_logger.set_target_logger(Rc::clone(&self.cli_logger));
        self.buffer_logger.flush();
    }
}

pub fn create_service() -> LogService {
    let file_logger: Rc<dyn AnyLogger> = Rc::new(CliLogger::new());
    let buffer_logger: Rc<dyn AnyLogger> = Rc::new(BufferLogger::new(5000)); // TODO: move this value to config/constant
    let cli_logger: Rc<dyn AnyLogger> = Rc::new(CliLogger::new());
    return LogService::new(file_logger, buffer_logger, cli_logger);
}


pub trait AnyLogger {
    fn trace(&self, message: &str);
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warning(&self, message: &str);
    fn error(&self, message: &str);

    fn flush(&self);
    
    
    fn set_level(&mut self, level: LogLevel);

    fn set_format(&mut self, formatter: Box<dyn Fn(&str, LogLevel) -> String>);
    
    fn enable(&mut self);
    fn disable(&mut self);

    // Set target to re-direct logs to some other logger (such, as, from buffer to others...)
    fn set_target_logger(&mut self, logger: Rc<dyn AnyLogger>);
}

#[derive(Debug)]
pub enum LogLevel {
    Off,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

struct CliLogger {
    enabled: bool,
    level: LogLevel,
    formatter: Box<dyn Fn(&str, LogLevel) -> String>,
}

impl CliLogger {
    pub fn new() -> Self {
        CliLogger::default()
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
            formatter: Box::new(|message, level| format!("[{:?}] {}", level, message)),
            enabled: true,
        }
    }
}

impl AnyLogger for CliLogger {
    fn trace(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Trace) {
            println!("{}", (self.formatter)(message, LogLevel::Trace));
        }
    }

    fn debug(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
            println!("{}", (self.formatter)(message, LogLevel::Debug));
        }
    }

    fn info(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            println!("{}", (self.formatter)(message, LogLevel::Info));
        }
    }

    fn warning(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace | LogLevel::Warning ) {
            println!("{}", (self.formatter)(message, LogLevel::Warning));
        }
    }

    fn error(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace | LogLevel::Warning | LogLevel::Error ) {
            println!("{}", (self.formatter)(message, LogLevel::Error));
        }
    }

    fn flush(&self) {
        todo!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn set_format(&mut self, formatter: Box<dyn Fn(&str, LogLevel) -> String>) {
        self.formatter = formatter;
    }

    fn enable(&mut self) {
        self.enabled = true
    }

    fn disable(&mut self) {
        self.enabled = false
    }

    fn set_target_logger(&mut self, logger: Rc<dyn AnyLogger>) {        unimplemented!()
    }
}

pub struct BufferLogger {
    buffer: VecDeque<BufferedLogEntry>, 
    max_buffer_size: usize,
    enabled: bool,
    level: LogLevel,
    target_logger: Option<Weak<dyn AnyLogger>>,
}

pub struct BufferedLogEntry {
    level: LogLevel,
    message: String,   
}

impl BufferedLogEntry {
    fn new(level: LogLevel, msg: String) -> BufferedLogEntry {
        Self {
            level: level,
            message: msg,
        }
    }
}

impl BufferLogger {
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            max_buffer_size,
            enabled: true,
            level: LogLevel::Trace,
            target_logger: None, // Initially no target logger
        }
    }

    // Buffer log entries as (level, message) pairs
    fn buffer_log(&mut self, level: LogLevel, message: String) {
        if self.buffer.len() >= self.max_buffer_size {
            self.buffer.pop_front(); // Discard the oldest log when the buffer is full
        }
        self.buffer.push_back(BufferedLogEntry::new(level, message));
    }
}

impl AnyLogger for BufferLogger {
    fn trace(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Trace) {
            self.buffer_log(LogLevel::Trace, message.to_string());
        }
    }

    fn debug(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Debug, message.to_string());
        }
    }

    fn info(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Info, message.to_string());
        }
    }

    fn warning(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Warning, message.to_string());
        }
    }

    fn error(&self, message: &str) {
        if self.enabled && matches!(self.level, LogLevel::Error | LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Error, message.to_string());
        }
    }

    fn flush(&self) {
        if let Some(ref logger) = self.target_logger {
            while let Some(log_entry) = self.buffer.pop_front() {
                match log_entry.level {
                    LogLevel::Trace => logger.trace(&log_entry.message),
                    LogLevel::Debug => logger.debug(&log_entry.message),
                    LogLevel::Info => logger.info(&log_entry.message),
                    LogLevel::Warning => logger.warning(&log_entry.message),
                    LogLevel::Error => logger.error(&log_entry.message),
                    LogLevel::Off => ()
                }
            }
        }
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn set_format(&mut self, formatter: Box<dyn Fn(&str, LogLevel) -> String>) {
        // Do nothing, as the format will be set by more specific loggers
        unimplemented!()
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn set_target_logger(&mut self, logger: Rc<dyn AnyLogger>) {
        self.target_logger = Some(Rc::downgrade(&logger));
    }
}