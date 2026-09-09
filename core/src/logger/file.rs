use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::utils::time::now_millis;

use super::{LogLevel, Logger};

pub struct FileLogger {
    file: Mutex<File>,
    max_size: u64,
    path: PathBuf,
}

impl FileLogger {
    pub fn new(path: impl AsRef<Path>, max_size: u64) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(FileLogger {
            file: Mutex::new(file),
            max_size,
            path,
        })
    }

    fn rotate_if_needed(&self) {
        let file = self.file.lock().unwrap();
        if file.metadata().map(|m| m.len()).unwrap_or(0) > self.max_size {
            let timestamp = now_millis();
            let rotated = self.path.with_extension(format!(
                "{}.{}",
                self.path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("log"),
                timestamp
            ));
            drop(file);
            let _ = std::fs::rename(&self.path, &rotated);
            if let Ok(new_file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)
            {
                *self.file.lock().unwrap() = new_file;
            }
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, target: &str, message: &str) {
        self.rotate_if_needed();
        let mut file = self.file.lock().unwrap();
        let ts = now_millis();
        let _ = writeln!(file, "[{ts}] [{:<5}] {target}: {message}", level.as_str());
    }

    fn flush(&self) {
        let mut file = self.file.lock().unwrap();
        let _ = file.flush();
    }
}
