use crate::logger::{LogLevel, Logger};
use std::ffi::CString;

pub struct SyslogLogger;

impl SyslogLogger {
    pub fn new(ident: &str) -> Self {
        let ident = CString::new(ident).unwrap_or_default();
        unsafe {
            openlog(ident.as_ptr() as *const u8, LOG_PID, LOG_USER);
        }
        SyslogLogger
    }

    fn priority(&self, level: LogLevel) -> i32 {
        match level {
            LogLevel::Trace | LogLevel::Debug => LOG_DEBUG,
            LogLevel::Info => LOG_INFO,
            LogLevel::Warn => LOG_WARNING,
            LogLevel::Error => LOG_ERR,
        }
    }
}

impl Drop for SyslogLogger {
    fn drop(&mut self) {
        unsafe {
            closelog();
        }
    }
}

impl Logger for SyslogLogger {
    fn log(&self, level: LogLevel, _target: &str, message: &str) {
        let msg = CString::new(message).unwrap_or_default();
        unsafe {
            syslog(self.priority(level), b"%s\0".as_ptr() as *const _, msg.as_ptr());
        }
    }

    fn flush(&self) {}
}

const LOG_PID: i32 = 0x01;
const LOG_USER: i32 = 1 << 3;
const LOG_ERR: i32 = 3;
const LOG_WARNING: i32 = 4;
const LOG_INFO: i32 = 6;
const LOG_DEBUG: i32 = 7;

extern "C" {
    fn openlog(ident: *const u8, option: i32, facility: i32);
    fn closelog();
    fn syslog(priority: i32, format: *const u8, ...);
}
