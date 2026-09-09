use waydri_core::plugin::{
    PluginLoader, PluginManager, PluginManifest, PluginSandbox, SandboxCapability, SandboxPolicy,
    WaydriPlugin,
};

struct TestPlugin {
    ticks: u64,
    started: bool,
}

impl WaydriPlugin for TestPlugin {
    fn name(&self) -> &str {
        "test_plugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn on_start(&mut self) {
        self.started = true;
    }

    fn on_stop(&mut self) {}

    fn on_tick(&mut self, delta_ms: u64) {
        self.ticks += delta_ms;
    }
}

#[test]
fn manager_starts_and_ticks_plugins() {
    let mut manager = PluginManager::new(PluginLoader::new(PluginLoader::default_dir()));
    let plugin = TestPlugin { ticks: 0, started: false };
    manager.start(Box::new(plugin));
    assert_eq!(manager.running_names(), vec!["test_plugin"]);
    manager.tick(16);
    assert_eq!(manager.running_names().len(), 1);
    manager.stop("test_plugin");
    assert!(manager.running_names().is_empty());
}

#[test]
fn manifest_from_json_round_trips() {
    let json = r#"{"name":"sample","version":"0.1.0","description":"sample plugin","entry":"plugin.lua","capabilities":["network"]}"#;
    let parsed = PluginManifest::from_json(json).unwrap();
    assert_eq!(parsed.name, "sample");
    assert_eq!(parsed.entry, "plugin.lua");
    assert!(parsed.can("network"));
    assert!(!parsed.can("filesystem"));
}

#[test]
fn sandbox_denies_unpermitted_capability() {
    let policy = SandboxPolicy::from_strings(&["rpc".to_string()]);
    let sandbox = PluginSandbox::new().with_policy(policy);
    assert!(sandbox.check(SandboxCapability::Network).is_err());
    assert!(sandbox.check(SandboxCapability::Rpc).is_ok());
}

#[test]
fn capability_names_round_trip() {
    for cap in [
        SandboxCapability::Network,
        SandboxCapability::FilesystemRead,
        SandboxCapability::FilesystemWrite,
        SandboxCapability::CompositorControl,
        SandboxCapability::Input,
        SandboxCapability::Rpc,
    ] {
        assert_eq!(SandboxCapability::from_str(cap.as_str()), cap);
    }
}

#[test]
fn unknown_capability_maps_to_none() {
    assert_eq!(SandboxCapability::from_str("nope"), SandboxCapability::None);
}