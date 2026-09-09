use std::fs;
use std::path::{Path, PathBuf};

use super::Config;

#[derive(Debug)]
pub enum LoadError {
    NotFound,
    Io(std::io::Error),
    Parse(String),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::NotFound => write!(f, "config file not found"),
            LoadError::Io(e) => write!(f, "io error: {e}"),
            LoadError::Parse(msg) => write!(f, "parse error: {msg}"),
        }
    }
}

impl std::error::Error for LoadError {}

pub struct ConfigLoader {
    search_paths: Vec<PathBuf>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        ConfigLoader {
            search_paths: Vec::new(),
        }
    }

    pub fn with_search_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.search_paths.push(path.into());
        self
    }

    pub fn default_search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();
        if let Ok(home) = std::env::var("HOME") {
            paths.push(PathBuf::from(home).join(".config").join("waydri"));
        }
        paths.push(PathBuf::from("/etc/waydri"));
        paths.push(PathBuf::from("."));
        paths
    }

    pub fn load(&self) -> Result<Config, LoadError> {
        let paths = if self.search_paths.is_empty() {
            Self::default_search_paths()
        } else {
            self.search_paths.clone()
        };
        for dir in &paths {
            let candidate = dir.join("waydri.toml");
            if candidate.exists() {
                return self.load_from(&candidate);
            }
            let json = dir.join("waydri.json");
            if json.exists() {
                return self.load_from(&json);
            }
        }
        Err(LoadError::NotFound)
    }

    pub fn load_from(&self, path: &Path) -> Result<Config, LoadError> {
        let contents = fs::read_to_string(path).map_err(LoadError::Io)?;
        let config = match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => super::parser::parse_toml(&contents).map_err(LoadError::Parse)?,
            _ => super::parser::parse_json(&contents).map_err(LoadError::Parse)?,
        };
        Ok(config)
    }

    pub fn load_str(&self, contents: &str, is_json: bool) -> Result<Config, LoadError> {
        let config = if is_json {
            super::parser::parse_json(contents).map_err(LoadError::Parse)?
        } else {
            super::parser::parse_toml(contents).map_err(LoadError::Parse)?
        };
        Ok(config)
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}