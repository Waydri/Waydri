use super::api::{PluginManifest, WaydriPlugin};
use super::loader::PluginLoader;

pub struct PluginManager {
    pub loader: PluginLoader,
    pub manifests: Vec<PluginManifest>,
    pub enabled: Vec<String>,
    pub running: Vec<Box<dyn WaydriPlugin>>,
}

impl PluginManager {
    pub fn new(loader: PluginLoader) -> Self {
        PluginManager {
            loader,
            manifests: Vec::new(),
            enabled: Vec::new(),
            running: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        self.manifests = self.loader.list_manifests();
    }

    pub fn enable(&mut self, name: &str) -> bool {
        if !self.enabled.contains(&name.to_string()) && self.loader.load_manifest(name).is_some() {
            self.enabled.push(name.to_string());
            return true;
        }
        false
    }

    pub fn disable(&mut self, name: &str) -> bool {
        let before = self.enabled.len();
        self.enabled.retain(|n| n != name);
        self.stop(name);
        self.enabled.len() != before
    }

    pub fn start(&mut self, plugin: Box<dyn WaydriPlugin>) {
        let mut plugin = plugin;
        plugin.on_start();
        self.running.push(plugin);
    }

    pub fn stop(&mut self, name: &str) {
        self.running.retain_mut(|plugin| {
            if plugin.name() == name {
                plugin.on_stop();
                false
            } else {
                true
            }
        });
    }

    pub fn tick(&mut self, delta_ms: u64) {
        for plugin in self.running.iter_mut() {
            plugin.on_tick(delta_ms);
        }
    }

    pub fn running_names(&self) -> Vec<String> {
        self.running.iter().map(|p| p.name().to_string()).collect()
    }

    pub fn available(&self) -> Vec<String> {
        self.manifests.iter().map(|m| m.name.clone()).collect()
    }
}