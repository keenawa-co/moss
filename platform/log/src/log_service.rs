use std::{cell::RefCell, collections::VecDeque, default, fs::{File, OpenOptions}, io::{BufWriter, Write}, path::PathBuf, rc::{Rc, Weak}, sync::{Arc, Mutex}};

use chrono::Utc;

pub struct LogService {
    file_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>,
    buffer_logger: Arc<Mutex<dyn BufferableLogger + Send + Sync + 'static>>,
    cli_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>,
}

impl LogService {
    pub fn new(
        file_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>, 
        buffer_logger: Arc<Mutex<dyn BufferableLogger + Send + Sync + 'static>>, 
        cli_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>>) -> Self {
        Self {
            file_logger,
            buffer_logger,
            cli_logger,
        }
    }

    // TODO: replace unwrap with match (in all functions)
    pub fn trace(&self, message: &str) {
        self.buffer_logger.lock().unwrap().trace(message);
        self.file_logger.lock().unwrap().trace(message);
    }

    pub fn debug(&self, message: &str) {
        self.buffer_logger.lock().unwrap().debug(message);
        self.file_logger.lock().unwrap().debug(message);
    }

    pub fn info(&self, message: &str) {
        self.buffer_logger.lock().unwrap().info(message);
        self.file_logger.lock().unwrap().info(message);
    }
    
    pub fn warning(&self, message: &str) {
        self.buffer_logger.lock().unwrap().warning(message);
        self.file_logger.lock().unwrap().warning(message);
    }
    
    pub fn error(&self, message: &str) {
        self.buffer_logger.lock().unwrap().error(message);
        self.file_logger.lock().unwrap().error(message);
    }

    // method used only for testing
    pub fn flush_buffer_logger_to_cli(&mut self) {
        let mut buffered_logs = self.buffer_logger.lock().unwrap().drain_logs();
        
        while let Some(log_entry) = buffered_logs.pop_front() {
            match log_entry.level {
                LogLevel::Trace => self.cli_logger.lock().unwrap().trace(&log_entry.message),
                LogLevel::Debug => self.cli_logger.lock().unwrap().debug(&log_entry.message),
                LogLevel::Info => self.cli_logger.lock().unwrap().info(&log_entry.message),
                LogLevel::Warning => self.cli_logger.lock().unwrap().warning(&log_entry.message),
                LogLevel::Error => self.cli_logger.lock().unwrap().error(&log_entry.message),
                LogLevel::Off => ()
            }
        }
    }
}

pub fn create_service() -> Result<LogService, std::io::Error> {
    let file_logger = FileLogger::new(
        PathBuf::from("/home/krail/projects/moss/logs/testlog.log"),
        50, // Max file size in bytes
    )?;

    let file_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(Mutex::new(file_logger));
    let buffer_logger: Arc<Mutex<dyn BufferableLogger + Send + Sync + 'static>> = Arc::new(Mutex::new(BufferLogger::new(5000))); // TODO: move this value to config/constant
    let cli_logger: Arc<Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(Mutex::new(CliLogger::new()));
    
    Ok(LogService::new(file_logger, buffer_logger, cli_logger))
}


pub trait AnyLogger {
    // fn log(&mut self, level: LogLevel, message: &str);
    fn trace(&mut self, message: &str);
    fn debug(&mut self, message: &str);
    fn info(&mut self, message: &str);
    fn warning(&mut self, message: &str);
    fn error(&mut self, message: &str);

    fn flush(&mut self);
    
    
    fn set_level(&mut self, level: LogLevel);

    fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>);

    // determine if the logger is ready to be used (files are created, memory allocated, etc.)
    fn is_ready(&self) -> bool; 

    // Set target to re-direct logs to some other logger (such, as, from buffer to others...)
    // fn set_target_logger(&mut self, logger: Arc<Mutex<dyn AnyLogger>>);
}

pub trait Bufferable {
    fn drain_logs(&self) -> VecDeque<BufferedLogEntry>;
}

pub trait BufferableLogger: AnyLogger + Bufferable {}
impl<T: AnyLogger + Bufferable> BufferableLogger for T {}

#[derive(Debug,Clone)]
pub enum LogLevel {
    Off,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

// #[derive(Default)]
struct CliLogger {
    level: LogLevel,
    formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>,
}

impl CliLogger {
    pub fn new() -> Self {
        Self::default()
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
            formatter: Arc::new(|message, level| format!("[{:?}] {}", level, message)),
        }
    }
}

impl AnyLogger for CliLogger {
    fn trace(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Trace) {
            println!("{}", (self.formatter)(message, LogLevel::Trace));
        }
    }

    fn debug(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
            println!("{}", (self.formatter)(message, LogLevel::Debug));
        }
    }

    fn info(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            println!("{}", (self.formatter)(message, LogLevel::Info));
        }
    }

    fn warning(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace | LogLevel::Warning ) {
            println!("{}", (self.formatter)(message, LogLevel::Warning));
        }
    }

    fn error(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace | LogLevel::Warning | LogLevel::Error ) {
            println!("{}", (self.formatter)(message, LogLevel::Error));
        }
    }

    fn flush(&mut self) {
        todo!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>) {
        self.formatter = formatter;
    }

    fn is_ready(&self) -> bool {
        return true; // cli logger is always ready
    }

    // fn set_target_logger(&mut self, logger: Arc<Mutex<dyn AnyLogger>>) {
        // unimplemented!()
    // }

}

pub struct BufferLogger {
    buffer: VecDeque<BufferedLogEntry>, 
    max_buffer_size: usize,
    level: LogLevel,
}

#[derive(Clone)]
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
            level: LogLevel::Trace,
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
    fn trace(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Trace) {
            self.buffer_log(LogLevel::Trace, message.to_string());
        }
    }

    fn debug(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Debug, message.to_string());
        }
    }

    fn info(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Info, message.to_string());
        }
    }

    fn warning(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Warning, message.to_string());
        }
    }

    fn error(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Error | LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            self.buffer_log(LogLevel::Error, message.to_string());
        }
    }

    fn flush(&mut self) {
        // if let Some(target_logger) = &self.target_logger {
        //     let mut logger = target_logger.lock().unwrap(); 
        //     while let Some(log_entry) = self.buffer.pop_front() {
        //         match log_entry.level {
        //             LogLevel::Trace => logger.trace(&log_entry.message),
        //             LogLevel::Debug => logger.debug(&log_entry.message),
        //             LogLevel::Info => logger.info(&log_entry.message),
        //             LogLevel::Warning => logger.warning(&log_entry.message),
        //             LogLevel::Error => logger.error(&log_entry.message),
        //             LogLevel::Off => ()
        //         }
        //     }
        // }

        unimplemented!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>) {
        // Do nothing, as the format will be set by more specific loggers
        unimplemented!()
    }

    // fn set_target_logger(&mut self, logger: Arc<Mutex<dyn AnyLogger>>) {
    //     self.target_logger = Some(logger);
    // }

    fn is_ready(&self) -> bool {
        todo!();
    }
}

impl Bufferable for BufferLogger {
    fn drain_logs(&self) -> VecDeque<BufferedLogEntry> {
        self.buffer.clone()
    }
}

// pub struct FileLogger {
//     file_path: PathBuf,
//     max_file_size: usize,
//     current_size: usize,
//     file: BufWriter<File>,
//     level: LogLevel,
//     formatter: Box<dyn Fn(&str, LogLevel) -> String>,
// }

pub struct FileLogger {
    file: Option<File>,
    file_path: PathBuf,
    max_file_size: u64,
    current_file_size: u64,
    level: LogLevel,
    formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>,
}

impl FileLogger {
    pub fn new(file_path: PathBuf, max_file_size: u64) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        let current_file_size = file.metadata()?.len();

        Ok(Self {
            file: Some(file),
            file_path,
            max_file_size,
            current_file_size,
            level: LogLevel::Trace,
            formatter: Arc::new(|message, level| format!("[{:?}] {}", level, message)),
        })
    }

    fn rotate_log(&mut self) -> Result<(), std::io::Error> {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let rotated_file_name = format!(
            "{}_{}.log",
            self.file_path.file_stem().unwrap().to_str().unwrap(),
            timestamp
        );

        let rotated_file_path = self.file_path.with_file_name(rotated_file_name);

        // Rename the current log file
        std::fs::rename(&self.file_path, rotated_file_path)?;

        // Open a new file for logging
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        self.file = Some(new_file);
        self.current_file_size = 0;

        Ok(())
    }

    fn log_with_rotation(&mut self, message: &str, level: LogLevel) -> Result<(), std::io::Error> {
        if self.current_file_size >= self.max_file_size {
            self.rotate_log()?;
        }

        if let Some(file) = self.file.as_mut() {
            let formatted_message = (self.formatter)(message, level);
            let bytes_written = file.write(formatted_message.as_bytes())?;
            file.write(b"\n")?; // write a newline after the log message
            self.current_file_size += bytes_written as u64 + 1;
        }

        Ok(())
    }
}

impl AnyLogger for FileLogger {
    fn trace(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Trace) {
            let _ = self.log_with_rotation(message, LogLevel::Trace);
        }
    }

    fn debug(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
            let _ = self.log_with_rotation(message, LogLevel::Debug);
        }
    }

    fn info(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            let _ = self.log_with_rotation(message, LogLevel::Info);
        }
    }

    fn warning(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            let _ = self.log_with_rotation(message, LogLevel::Warning);
        }
    }

    fn error(&mut self, message: &str) {
        if matches!(self.level, LogLevel::Error | LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
            let _ = self.log_with_rotation(message, LogLevel::Error);
        }
    }

    fn flush(&mut self) {
        if let Some(file) = self.file.as_mut() {
            let _ = file.flush();
        }
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>) {
        self.formatter = formatter;
    }

    fn is_ready(&self) -> bool {
        self.file.is_some()
    }
}