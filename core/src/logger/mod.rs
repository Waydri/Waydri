pub mod android;
pub mod console;
pub mod file;
pub mod syslog;

pub use console::ConsoleLogger;
pub use file::FileLogger;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn from_str(s: &str) -> Option<LogLevel> {
        match s.to_ascii_lowercase().as_str() {
            "trace" => Some(LogLevel::Trace),
            "debug" => Some(LogLevel::Debug),
            "info" => Some(LogLevel::Info),
            "warn" | "warning" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    }
}

pub trait Logger: Send + Sync {
    fn log(&self, level: LogLevel, target: &str, message: &str);
    fn flush(&self);
}

pub struct SinkLogger {
    sinks: Vec<Box<dyn Logger>>,
    min_level: LogLevel,
}

impl SinkLogger {
    pub fn new(min_level: LogLevel) -> Self {
        SinkLogger {
            sinks: Vec::new(),
            min_level,
        }
    }

    pub fn add_sink(&mut self, sink: Box<dyn Logger>) {
        self.sinks.push(sink);
    }

    pub fn log(&self, level: LogLevel, target: &str, message: &str) {
        if level as u8 >= self.min_level as u8 {
            for sink in &self.sinks {
                sink.log(level, target, message);
            }
        }
    }
}
