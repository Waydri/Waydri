use waydri_core::logger::{ConsoleLogger, FileLogger, LogLevel, Logger};

#[test]
fn log_level_round_trip() {
    for level in [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ] {
        assert_eq!(LogLevel::from_str(&level.as_str().to_lowercase()), Some(level));
    }
}

#[test]
fn log_level_unknown() {
    assert_eq!(LogLevel::from_str("bogus"), None);
    assert_eq!(LogLevel::from_str("WARNING"), Some(LogLevel::Warn));
}

#[test]
fn levels_ordered() {
    assert!((LogLevel::Trace as u8) < (LogLevel::Error as u8));
}

#[test]
fn console_logger_writes() {
    let logger = ConsoleLogger::new();
    logger.log(LogLevel::Info, "test", "hello world");
    logger.flush();
}

#[test]
fn file_logger_rotates() {
    let path = "/tmp/waydri_logger_test.log";
    let _ = std::fs::remove_file(path);
    let logger = FileLogger::new(path, 128).unwrap();
    for i in 0..64 {
        logger.log(LogLevel::Debug, "loop", &format!("line {i:03}"));
    }
    logger.flush();
    assert!(std::path::Path::new(path).exists());
    for entry in std::fs::read_dir("/tmp").unwrap().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("waydri_logger_test.log.") {
            let _ = std::fs::remove_file(entry.path());
        }
    }
    let _ = std::fs::remove_file(path);
}

#[test]
fn dropped_file_logger_is_safe() {
    let path = "/tmp/waydri_logger_drop.log";
    {
        let logger = FileLogger::new(path, 4096).unwrap();
        logger.log(LogLevel::Error, "cleanup", "closing");
    }
    let _ = std::fs::remove_file(path);
}