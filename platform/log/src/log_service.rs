use std::default;

pub struct LogService {
    file_logger: Box<dyn AnyLogger>,
    buffer_logger: Box<dyn AnyLogger>,
    cli_logger: Box<dyn AnyLogger>,
}

impl LogService {
    pub fn new(file_logger: Box<dyn AnyLogger>, buffer_logger: Box<dyn AnyLogger>, cli_logger: Box<dyn AnyLogger>) -> Self {
        Self {
            file_logger,
            buffer_logger,
            cli_logger,
        }
    }

    pub fn trace(&self, message: &str) {
        self.cli_logger.trace(message);
    }

    pub fn debug(&self, message: &str) {
        self.cli_logger.debug(message);
    }

    pub fn info(&self, message: &str) {
        self.cli_logger.info(message);
    }
    
    pub fn warning(&self, message: &str) {
        self.cli_logger.warning(message);
    }
    
    pub fn error(&self, message: &str) {
        self.cli_logger.error(message);
    }
}

pub fn create_service() -> LogService {
    let file_logger: Box<dyn AnyLogger> = Box::new(CliLogger::new());
    let buffer_logger: Box<dyn AnyLogger> = Box::new(CliLogger::new());
    let cli_logger: Box<dyn AnyLogger> = Box::new(CliLogger::new());
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
}