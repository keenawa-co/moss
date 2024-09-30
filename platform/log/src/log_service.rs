use std::{borrow::Borrow, cell::RefCell, collections::{binary_heap, VecDeque}, default, fs::{File, OpenOptions}, io::{BufWriter, Write}, path::PathBuf, process::exit, rc::{Rc, Weak}, sync::{Arc, Mutex}};

use chrono::Utc;
use tracing::{debug, error, event, info, level_filters::LevelFilter, trace, warn, Level};
use tracing_appender::rolling::{self, daily};
use tracing_subscriber::{filter::DynFilterFn, fmt::{self, writer::BoxMakeWriter}, layer::{Layered, SubscriberExt}, util::SubscriberInitExt, EnvFilter, Layer, Registry};

pub struct LogService {
    file_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>,
    buffer_logger: Arc<parking_lot::Mutex<dyn BufferableLogger + Send + Sync + 'static>>,
    cli_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>,
}

impl LogService {
    pub fn new(
        file_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>, 
        buffer_logger: Arc<parking_lot::Mutex<dyn BufferableLogger + Send + Sync + 'static>>, 
        cli_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>>) -> Self {
        
        let service = Self {
            file_logger,
            buffer_logger,
            cli_logger,
        };

        service.init_tracing();
        
        return service
    }

    // TODO: replace unwrap with match (in all functions)
    pub fn trace(&self, message: &str) {
        self.cli_logger.lock().log(LogLevel::Trace, message);
    }

    pub fn debug(&self, message: &str) {
        self.cli_logger.lock().log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.cli_logger.lock().log(LogLevel::Info, message);
    }
    
    pub fn warning(&self, message: &str) {
        self.cli_logger.lock().log(LogLevel::Warning, message);
    }
    
    pub fn error(&self, message: &str) {
        self.cli_logger.lock().log(LogLevel::Error, message);
    }

    // method used only for testing
    pub fn flush_buffer_logger_to_cli(&mut self) {
        // let mut buffered_logs = self.buffer_logger.lock().drain_logs();
        
        // while let Some(log_entry) = buffered_logs.pop_front() {
        //     match log_entry.level {
        //         LogLevel::Trace => self.cli_logger.lock().trace(&log_entry.message),
        //         LogLevel::Debug => self.cli_logger.lock().debug(&log_entry.message),
        //         LogLevel::Info => self.cli_logger.lock().info(&log_entry.message),
        //         LogLevel::Warning => self.cli_logger.lock().warning(&log_entry.message),
        //         LogLevel::Error => self.cli_logger.lock().error(&log_entry.message),
        //         LogLevel::Off => ()
        //     }
        // }
    }

    fn init_tracing(&self) {
        // let file_appender = rolling::daily("/var/log/moss", "app.log");
    
        // let file_layer = tracing_subscriber::fmt::layer()
        //     .with_writer(BoxMakeWriter::new(file_appender))
        //     .with_ansi(false) // Disable color in log file
        //     .with_thread_ids(true) // Include thread IDs in the file logs
        //     .with_thread_names(true)
        //     .with_level(true) // Include log levels in the file logs
        //     .with_target(true) // Show the target (module path) in the file logs
        //     .event_format(tracing_subscriber::fmt::format().compact()) // Customize the log format for the file
        //     .with_filter(EnvFilter::new("TRACE"))
        //     .with_filter(DynFilterFn::new(|metadata, _cx| {
        //         match metadata.fields().field("file") {
        //             Some(field) => field.clone().to_string() == "file",
        //             None => false
        //         }
        //         // return metadata.target() == "file"
        //     }));
    
            // Setup console layer
        let console_layer = tracing_subscriber::fmt::layer()
            .with_ansi(true) // Enable color for console
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_level(true)
            .with_target(true)
            .event_format(tracing_subscriber::fmt::format().pretty()) // Customize the log format for the console
            .with_filter(EnvFilter::new("TRACE"))
            .with_filter(DynFilterFn::new(|metadata, _cx| {
                match metadata.fields().field("cli") {
                    Some(field) => field.clone().to_string() == "cli",
                    None => false
                }
            }));
    
        tracing_subscriber::registry()
            .with(console_layer)
            // .with(file_layer)
            .init();
    }
}

pub fn create_service() -> Result<LogService, std::io::Error> {
    let file_logger = FileLogger::new(
        PathBuf::from("/home/krail/projects/moss/logs/testlog.log"),
        50, // Max file size in bytes
    )?;

    let file_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(parking_lot::Mutex::new(file_logger));
    let buffer_logger: Arc<parking_lot::Mutex<dyn BufferableLogger + Send + Sync + 'static>> = Arc::new(parking_lot::Mutex::new(BufferLogger::new(5000))); // TODO: move this value to config/constant
    let cli_logger: Arc<parking_lot::Mutex<dyn AnyLogger + Send + Sync + 'static>> = Arc::new(parking_lot::Mutex::new(CliLogger::new()));
    
    Ok(LogService::new(file_logger, buffer_logger, cli_logger))
}


pub trait AnyLogger {
    // fn log(&mut self, level: LogLevel, message: &str);
    fn log(&mut self, level: LogLevel, message: &str);
    // fn trace(&mut self, message: &str);
    // fn debug(&mut self, message: &str);
    // fn info(&mut self, message: &str);
    // fn warning(&mut self, message: &str);
    // fn error(&mut self, message: &str);

    fn flush(&mut self);
    
    
    fn set_level(&mut self, level: LogLevel);

    // determine if the logger is ready to be used (files are created, memory allocated, etc.)
    fn is_ready(&self) -> bool; 

}

pub trait Bufferable {
    fn drain_logs(&self) -> VecDeque<BufferedLogEntry>;
}

pub trait BufferableLogger: AnyLogger + Bufferable {}
impl<T: AnyLogger + Bufferable> BufferableLogger for T {}

#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Off,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn to_tracing_level(&self) -> Level {
        match self {
            LogLevel::Off => Level::TRACE, // TODO: Disable logging
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warning => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

// #[derive(Default)]
struct CliLogger {
    level: LogLevel,
}

impl CliLogger {
    pub fn new() -> Self {
        Self::default()
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
        }
    }
}

impl AnyLogger for CliLogger {
    fn log(&mut self, level: LogLevel, message: &str) {
        if self.level < level {
            // do not log
            return
        }

        match level {
            LogLevel::Off => return, // ignore input
            LogLevel::Trace => event!(Level::TRACE, cli=true, message),
            LogLevel::Debug => event!(Level::DEBUG, cli=true, message),
            LogLevel::Info => event!(Level::INFO, cli=true, message),
            LogLevel::Warning => event!(Level::WARN, cli=true, message),
            LogLevel::Error => event!(Level::ERROR, cli=true, message),
        }
    }

    // fn trace(&mut self, message: &str) {
    //     if self.level >= LogLevel::Trace {
    //         event!(Level::TRACE, cli=true, message);
    //     }
    // }
    // fn debug(&mut self, message: &str) {
    //     if self.level >= LogLevel::Debug {
    //         event!(Level::DEBUG, cli=true, message);
    //     }
    // }
    // fn info(&mut self, message: &str) {
    //     if self.level >= LogLevel::Info {
    //         event!(Level::INFO, cli=true, message);
    //     }
    // }
    // fn warning(&mut self, message: &str) {
    //     if self.level >= LogLevel::Warning {
    //         event!(Level::WARN, cli=true, message);
    //     }
    // }
    // fn error(&mut self, message: &str) {
    //     if self.level >= LogLevel::Error {
    //         event!(Level::ERROR, cli=true, message);
    //     }
    // }

    fn flush(&mut self) {
        todo!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn is_ready(&self) -> bool {
        return true; // cli logger is always ready
    }

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
    fn log(&mut self, level: LogLevel, message: &str) {
        if self.level < level {
            // do not log
            return
        }

        match level {
            LogLevel::Off => return, // ignore input
            LogLevel::Trace => event!(Level::TRACE, buffer=true, message),
            LogLevel::Debug => event!(Level::DEBUG, buffer=true, message),
            LogLevel::Info => event!(Level::INFO, buffer=true, message),
            LogLevel::Warning => event!(Level::WARN, buffer=true, message),
            LogLevel::Error => event!(Level::ERROR, buffer=true, message),
        }
    }

    // fn trace(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Trace) {
    //         self.buffer_log(LogLevel::Trace, message.to_string());
    //     }
    // }

    // fn debug(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
    //         self.buffer_log(LogLevel::Debug, message.to_string());
    //     }
    // }

    // fn info(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
    //         self.buffer_log(LogLevel::Info, message.to_string());
    //     }
    // }

    // fn warning(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
    //         self.buffer_log(LogLevel::Warning, message.to_string());
    //     }
    // }

    // fn error(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Error | LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
    //         self.buffer_log(LogLevel::Error, message.to_string());
    //     }
    // }

    fn flush(&mut self) {
        unimplemented!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn is_ready(&self) -> bool {
        todo!();
    }

}

impl Bufferable for BufferLogger {
    fn drain_logs(&self) -> VecDeque<BufferedLogEntry> {
        self.buffer.clone()
    }
}

pub struct FileLogger {
    file: Option<File>,
    file_path: PathBuf,
    max_file_size: u64,
    current_file_size: u64,
    level: LogLevel,
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
        })
    }

    fn rotate_log(&mut self) -> Result<(), std::io::Error> {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let rotated_file_name = format!(
            "{:?}_{}.log",
            match self.file_path.file_stem() {
                Some(s) => s.to_str(),
                None => unimplemented!()
            },
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
            // let formatted_message = (self.formatter)(message, level);
            let bytes_written = file.write(message.as_bytes())?;
            file.write(b"\n")?; // write a newline after the log message
            self.current_file_size += bytes_written as u64 + 1;
        }

        Ok(())
    }
}

impl AnyLogger for FileLogger {
    fn log(&mut self, level: LogLevel, message: &str) {
        if self.level < level {
            // do not log
            return
        }

        match level {
            LogLevel::Off => return, // ignore input
            LogLevel::Trace => event!(Level::TRACE, file=true, message),
            LogLevel::Debug => event!(Level::DEBUG, file=true, message),
            LogLevel::Info => event!(Level::INFO, file=true, message),
            LogLevel::Warning => event!(Level::WARN, file=true, message),
            LogLevel::Error => event!(Level::ERROR, file=true, message),
        }
    }

    // fn trace(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Trace) {
    //         let _ = self.log_with_rotation(message, LogLevel::Trace);
    //     }
    // }

    // fn debug(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Debug | LogLevel::Trace) {
    //         let _ = self.log_with_rotation(message, LogLevel::Debug);
    //     }
    // }

    // fn info(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
    //         let _ = self.log_with_rotation(message, LogLevel::Info);
    //     }
    // }

    // fn warning(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
    //         let _ = self.log_with_rotation(message, LogLevel::Warning);
    //     }
    // }

    // fn error(&mut self, message: &str) {
    //     if matches!(self.level, LogLevel::Error | LogLevel::Warning | LogLevel::Info | LogLevel::Debug | LogLevel::Trace) {
    //         let _ = self.log_with_rotation(message, LogLevel::Error);
    //     }
    // }

    fn flush(&mut self) {
        if let Some(file) = self.file.as_mut() {
            let _ = file.flush();
        }
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    fn is_ready(&self) -> bool {
        self.file.is_some()
    }
}