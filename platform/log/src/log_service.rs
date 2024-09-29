use std::{borrow::Borrow, cell::RefCell, collections::{binary_heap, VecDeque}, default, fs::{File, OpenOptions}, io::{BufWriter, Write}, path::PathBuf, rc::{Rc, Weak}, sync::{Arc, Mutex}};

use chrono::Utc;
use tracing::{debug, error, info, level_filters::LevelFilter, trace, warn, Level};
use tracing_subscriber::{fmt, layer::{Layered, SubscriberExt}, util::SubscriberInitExt, EnvFilter, Layer, Registry};

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
        self.cli_logger.lock().trace(message);
    }

    pub fn debug(&self, message: &str) {
        self.cli_logger.lock().debug(message);
    }

    pub fn info(&self, message: &str) {
        self.cli_logger.lock().info(message);
    }
    
    pub fn warning(&self, message: &str) {
        self.cli_logger.lock().warning(message);
    }
    
    pub fn error(&self, message: &str) {
        self.cli_logger.lock().error(message);
    }

    // method used only for testing
    pub fn flush_buffer_logger_to_cli(&mut self) {
        let mut buffered_logs = self.buffer_logger.lock().drain_logs();
        
        while let Some(log_entry) = buffered_logs.pop_front() {
            match log_entry.level {
                LogLevel::Trace => self.cli_logger.lock().trace(&log_entry.message),
                LogLevel::Debug => self.cli_logger.lock().debug(&log_entry.message),
                LogLevel::Info => self.cli_logger.lock().info(&log_entry.message),
                LogLevel::Warning => self.cli_logger.lock().warning(&log_entry.message),
                LogLevel::Error => self.cli_logger.lock().error(&log_entry.message),
                LogLevel::Off => ()
            }
        }
    }

    fn init_tracing(&self) {
        let cli_layer = fmt::layer::<CliLogger>()
        .with_writer(|| {
            let cli_logger = CliLogger { level: LogLevel::Trace };
            Box::new(cli_logger)
        });

        // Create a subscriber that includes the cli_layer
        // let subscriber = Registry::default().with(cli_layer);

        // Create a subscriber that includes the cli_layer
        // let subscriber = Registry::default().with(cli_layer);

        // Set the subscriber as the global default
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
    
        // Set the subscriber as the global default
        // .filter(tracing_subscriber::filter::LevelFilter::INFO); // Adjust the log level here

        // let cli_layer = tracing_subscriber::fmt::layer()
        // .with_writer(std::io::stdout)
        // .with_filter(LevelFilter::from_level(Level::TRACE))
        // .with_filter(tracing_subscriber::filter::FilterFn::new(|metadata| {
        //     metadata.target() == "cli_target" // Only log this target for CLI
        // }));
    
        // let file_layer = tracing_subscriber::fmt::layer()
        //     .with_writer(|| std::fs::File::create("logfile.log"))
        //     .with_filter(LevelFilter::from_level(Level::TRACE))
        //     .with_filter(tracing_subscriber::filter::FilterFn::new(|metadata| {
        //         metadata.target() == "file_target" // Only log this target for file
        //     }));
        
        // let buffer_layer = tracing_subscriber::fmt::layer()
        //     .with_writer(std::io::sink)
        //     .with_filter(LevelFilter::from_level(Level::TRACE))
        //     .with_filter(tracing_subscriber::filter::FilterFn::new(|metadata| {
        //         metadata.target() == "buffer_target" // Only log this target for buffer
        //     }));

        // Create the base registry
        let mut registry = tracing_subscriber::registry()
            .with(cli_layer)
            .init();
        //     .with(buffer_layer)
        //     .init();



         // Log examples with specific targets
        // info!(target: "cli_target", "This is a log message for the CLI layer");
        // warn!(target: "file_target", "This is a warning message for the file layer");
        // debug!(target: "buffer_target", "This is a debug message for the buffer layer");
        // info!("This is a log message for all layers"); // This goes to all layers with trace level
    
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
    fn trace(&mut self, message: &str);
    fn debug(&mut self, message: &str);
    fn info(&mut self, message: &str);
    fn warning(&mut self, message: &str);
    fn error(&mut self, message: &str);

    fn flush(&mut self);
    
    
    fn set_level(&mut self, level: LogLevel);

    // fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>);

    // determine if the logger is ready to be used (files are created, memory allocated, etc.)
    fn is_ready(&self) -> bool; 

    // fn get_tracing_layer(&self) -> tracing_subscriber::fmt::Layer<tracing_subscriber::Registry>;
    // Set target to re-direct logs to some other logger (such, as, from buffer to others...)
    // fn set_target_logger(&mut self, logger: Arc<Mutex<dyn AnyLogger>>);
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
    // formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>,
}

impl CliLogger {
    pub fn new() -> Self {
        Self::default()
    }

    fn default() -> Self {
        Self {
            level: LogLevel::Trace,
            // formatter: Arc::new(|message, level| format!("[{:?}] {}", level, message)),
        }
    }
}

impl AnyLogger for CliLogger {
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
        todo!()
    }

    fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    // fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>) {
    //     self.formatter = formatter;
    // }

    fn is_ready(&self) -> bool {
        return true; // cli logger is always ready
    }

    // fn set_target_logger(&mut self, logger: Arc<Mutex<dyn AnyLogger>>) {
        // unimplemented!()
    // }
    // fn get_tracing_layer(&self) -> Filtered<tracing_subscriber::fmt::Layer<_, _, _, fn() -> Stdout {stdout}>, LevelFilter, _> {
    //     let level = self.level.to_tracing_level();

    //     tracing_subscriber::fmt::layer()
    //         .with_writer(std::io::stdout)
    //         .with_filter(LevelFilter::from_level(level)) 
    
    // }
}

impl Write for CliLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // Convert bytes to a string
        let message = String::from_utf8_lossy(buf);
        
        // Log the message according to the log level
        if self.level >= LogLevel::Trace {
            println!("TRACE: {}", message); // Directly write to stdout or handle as desired
        }
        
        // Log other levels similarly
        Ok(buf.len()) // Return the number of bytes written
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // Implement flush if necessary
        Ok(())
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
        //     let mut logger = target_logger.lock(); 
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

    // fn set_format(&mut self, formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>) {
    //     // Do nothing, as the format will be set by more specific loggers
    //     unimplemented!()
    // }

    // fn set_target_logger(&mut self, logger: Arc<Mutex<dyn AnyLogger>>) {
    //     self.target_logger = Some(logger);
    // }

    fn is_ready(&self) -> bool {
        todo!();
    }

    // fn get_tracing_layer(&self) -> Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync> {
    //     let level = self.level.to_tracing_level();

    //     Box::new(
    //         tracing_subscriber::fmt::layer()
    //         .with_writer(std::io::sink) // Buffer doesn't write directly to a destination
    //         .with_filter(LevelFilter::from_level(level)), // Convert `Level` to `LevelFilter`
    //     ) as Box<dyn Layer<_> + Send + Sync>
    // }
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
    // formatter: Arc<dyn Fn(&str, LogLevel) -> String + Send + Sync + 'static>,
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
            // formatter: Arc::new(|message, level| format!("[{:?}] {}", level, message)),
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

    fn is_ready(&self) -> bool {
        self.file.is_some()
    }
}