pub struct LogService {

}

pub trait AnyLogger {
    fn trace(&self, message: &str);
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warning(&self, message: &str);
    fn error(&self, message: &str);

    fn flush(&self);
    
    
    fn set_level(&self, level: LogLevel);

    fn set_format(&self, formatter: Box<dyn Fn(&str, LogLevel) -> String>);
    
    fn enable(&self);
    fn disable(&self);
}

pub enum LogLevel {
    Off,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}