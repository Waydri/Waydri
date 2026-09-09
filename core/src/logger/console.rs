use super::{LogLevel, Logger};

pub struct ConsoleLogger {
    use_color: bool,
}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {
            use_color: std::io::stdout().is_terminal(),
        }
    }

    pub fn with_color(mut self, enabled: bool) -> Self {
        self.use_color = enabled;
        self
    }

    fn color_for(&self, level: LogLevel) -> &'static str {
        match level {
            LogLevel::Trace => "\x1b[90m",
            LogLevel::Debug => "\x1b[34m",
            LogLevel::Info => "\x1b[32m",
            LogLevel::Warn => "\x1b[33m",
            LogLevel::Error => "\x1b[31m",
        }
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, target: &str, message: &str) {
        if self.use_color {
            let color = self.color_for(level);
            println!("{color}[{:<5}]\x1b[0m {target}: {message}", level.as_str());
        } else {
            println!("[{:<5}] {target}: {message}", level.as_str());
        }
    }

    fn flush(&self) {
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }
}

trait IsTerminal {
    fn is_terminal(&self) -> bool;
}

impl IsTerminal for std::io::Stdout {
    fn is_terminal(&self) -> bool {
        libc_isatty(1)
    }
}

extern "C" {
    fn isatty(fd: i32) -> i32;
}

fn libc_isatty(fd: i32) -> bool {
    unsafe { isatty(fd) == 1 }
}
