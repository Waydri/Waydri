use std::fs;
use std::path::{Path, PathBuf};

use super::api::PluginManifest;

pub struct PluginLoader {
    pub plugins_dir: PathBuf,
}

impl PluginLoader {
    pub fn new(plugins_dir: PathBuf) -> Self {
        PluginLoader { plugins_dir }
    }

    pub fn default_dir() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home)
                .join(".config")
                .join("waydri")
                .join("plugins")
        } else {
            PathBuf::from("/etc/waydri/plugins")
        }
    }

    pub fn list_manifests(&self) -> Vec<PluginManifest> {
        let mut manifests = Vec::new();
        let Ok(entries) = fs::read_dir(&self.plugins_dir) else {
            return manifests;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Ok(manifest) = PluginManifest::from_json(&contents) {
                    manifests.push(manifest);
                }
            }
        }
        manifests.sort_by(|a, b| a.name.cmp(&b.name));
        manifests
    }

    pub fn load_manifest(&self, name: &str) -> Option<PluginManifest> {
        let path = self.plugins_dir.join(format!("{name}.json"));
        let contents = fs::read_to_string(path).ok()?;
        PluginManifest::from_json(&contents).ok()
    }

    pub fn read_entry_source(&self, manifest: &PluginManifest) -> Option<String> {
        let entry: PathBuf = manifest.entry.clone().into();
        let candidate = if entry.is_absolute() {
            entry
        } else {
            self.plugins_dir.join(&manifest.entry)
        };
        fs::read_to_string(candidate).ok()
    }

    pub fn widget_watch_dirs(&self) -> Vec<PathBuf> {
        vec![self.plugins_dir.join("widgets")]
    }
}

pub fn home_config_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config").join("waydri")
    } else {
        PathBuf::from("/etc/waydri")
    }
}

pub fn path_exports(path: &Path) -> Vec<PathBuf> {
    let mut exports = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                exports.extend(path_exports(&entry_path));
            } else if entry_path.extension().and_then(|e| e.to_str())
                == Some("so")
            {
                exports.push(entry_path);
            }
        }
    }
    exports
}