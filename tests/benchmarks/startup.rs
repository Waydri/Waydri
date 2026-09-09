use std::time::Instant;

use waydri_core::config::{Config, ConfigLoader};

#[test]
fn default_config_instantiation() {
    let start = Instant::now();
    for _ in 0..10000 {
        let config = Config::default();
        assert_eq!(config.general.name, "Waydri");
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}

#[test]
fn toml_parse_throughput() {
    let source = r#"
[general]
name = "waydri"
max_fps = 120
border_size = 2
gaps = 8
log_level = "info"
"#;
    let loader = ConfigLoader::new();
    let start = Instant::now();
    for _ in 0..2000 {
        let config = loader.load_str(source, false).unwrap();
        assert_eq!(config.general.max_fps, 120);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}

#[test]
fn flat_config_flatten_cost() {
    let source = "[general]\nname = \"waydri\"\ngaps = 8\n[animation]\nenabled = true\n";
    let start = Instant::now();
    for _ in 0..2000 {
        let flat = waydri_core::config::parser::FlatConfig::from_toml(source);
        assert_eq!(flat.get("general.name"), Some("waydri"));
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}

#[test]
fn layout_manager_arrange_threshold() {
    let manager = waydri_core::layout::LayoutManager::with_default();
    let rect = waydri_core::utils::Rect::new(0, 0, 1920, 1080);
    let ids: Vec<_> = (1..=50).map(waydri_core::window::WindowId).collect();
    let start = Instant::now();
    for _ in 0..200 {
        let rects = manager.arrange("master_stack", rect, &ids);
        assert_eq!(rects.len(), 50);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}