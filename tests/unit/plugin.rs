use std::path::PathBuf;

use waydri_core::plugin::{
    API_VERSION, PluginLoader, PluginManager, PluginManifest, PluginSandbox, SandboxCapability,
    SandboxPolicy, WaydriPlugin,
};

struct TestPlugin {
    started: bool,
    ticks: u64,
}

impl TestPlugin {
    fn new() -> Self {
        TestPlugin { started: false, ticks: 0 }
    }
}

impl WaydriPlugin for TestPlugin {
    fn name(&self) -> &str {
        "test_plugin"
    }
    fn version(&self) -> &str {
        "0.1.0"
    }
    fn on_start(&mut self) {
        self.started = true;
    }
    fn on_stop(&mut self) {}
    fn on_tick(&mut self, _delta_ms: u64) {
        self.ticks += 1;
    }
}

#[test]
fn plugin_trait_callback_flow() {
    let mut plugin = TestPlugin::new();
    plugin.on_start();
    assert!(plugin.started);
    plugin.on_tick(16);
    plugin.on_tick(16);
    assert_eq!(plugin.ticks, 2);
}

#[test]
fn api_version_defined() {
    assert_eq!(API_VERSION, 1);
}

#[test]
fn manifest_round_trip() {
    let json = r#"{"name":"brightness","version":"1.0.0","description":"control brightness","entry":"brightness.rs","capabilities":["input","rpc"]}"#;
    let parsed = PluginManifest::from_json(json).unwrap();
    assert_eq!(parsed.name, "brightness");
    assert!(parsed.can("input"));
    assert!(!parsed.can("network"));
}

#[test]
fn loader_missing_dir_empty() {
    let loader = PluginLoader::new(PathBuf::from("/nonexistent/waydri/plugins"));
    assert!(loader.list_manifests().is_empty());
    assert!(loader.load_manifest("anything").is_none());
}

#[test]
fn manager_enable_disable() {
    let loader = PluginLoader::new(PathBuf::from("/nonexistent/plugins"));
    let mut manager = PluginManager::new(loader);
    manager.refresh();
    assert_eq!(manager.enable("ghost"), false);
    assert_eq!(manager.disable("ghost"), false);
}

#[test]
fn manager_run_plugin() {
    let loader = PluginLoader::new(PathBuf::from("/nonexistent/plugins"));
    let mut manager = PluginManager::new(loader);
    let plugin = TestPlugin::new();
    manager.start(Box::new(plugin));
    manager.tick(16);
    assert_eq!(manager.running_names(), vec!["test_plugin"]);
    manager.stop("test_plugin");
    assert!(manager.running_names().is_empty());
}

#[test]
fn sandbox_policy_permits() {
    let policy = SandboxPolicy::from_strings(&["input".to_string(), "rpc".to_string()]);
    assert!(policy.permits(SandboxCapability::Input));
    assert!(!policy.permits(SandboxCapability::Network));
    let sandbox = PluginSandbox::new().with_policy(policy);
    assert!(sandbox.check(SandboxCapability::Input).is_ok());
    assert!(sandbox.check(SandboxCapability::Network).is_err());
}

#[test]
fn capability_names() {
    assert_eq!(SandboxCapability::CompositorControl.as_str(), "compositor");
    assert_eq!(SandboxCapability::from_str("fs_write"), SandboxCapability::FilesystemWrite);
    assert_eq!(SandboxCapability::from_str("fs_read"), SandboxCapability::FilesystemRead);
}