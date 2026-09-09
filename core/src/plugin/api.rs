use serde::{Deserialize, Serialize};

pub trait WaydriPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_start(&mut self);
    fn on_stop(&mut self);
    fn on_tick(&mut self, _delta_ms: u64) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub entry: String,
    pub capabilities: Vec<String>,
}

impl PluginManifest {
    pub fn from_json(input: &str) -> Result<PluginManifest, String> {
        serde_json::from_str(input)
            .map_err(|error| format!("invalid plugin manifest: {error}"))
    }

    pub fn can(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

pub const API_VERSION: u32 = 1;

pub fn describe() -> &'static str {
    "plugin entry point and manifest types"
}