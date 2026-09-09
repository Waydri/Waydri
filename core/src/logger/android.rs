use super::{LogLevel, Logger};

pub struct AndroidLogger {
    tag: String,
}

impl AndroidLogger {
    pub fn new(tag: impl Into<String>) -> Self {
        AndroidLogger { tag: tag.into() }
    }

    fn priority(&self, level: LogLevel) -> i32 {
        match level {
            LogLevel::Trace => 2,
            LogLevel::Debug => 3,
            LogLevel::Info => 4,
            LogLevel::Warn => 5,
            LogLevel::Error => 6,
        }
    }
}

impl Logger for AndroidLogger {
    fn log(&self, level: LogLevel, target: &str, message: &str) {
        let full = format!("{target}: {message}");
        unsafe {
            __android_log_write(self.priority(level), self.tag.as_ptr(), full.as_ptr());
        }
    }

    fn flush(&self) {}
}

extern "C" {
    fn __android_log_write(prio: i32, tag: *const u8, text: *const u8) -> i32;
}
