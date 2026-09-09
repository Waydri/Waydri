use waydri_core::config::{Config, ConfigLoader, LoadError};

#[test]
fn defaults_are_sane() {
    let cfg = Config::default();
    assert_eq!(cfg.general.name, "Waydri");
    assert_eq!(cfg.general.max_fps, 120);
    assert_eq!(cfg.layout.default, "master_stack");
    assert_eq!(cfg.effects.blur_radius, 8.0);
    assert_eq!(cfg.ipc.socket_path, "/tmp/waydri.sock");
}

#[test]
fn loader_parses_toml() {
    let loader = ConfigLoader::new();
    let cfg = loader.load_str(
        "[general]\nname = \"Test\"\nmax_fps = 60\nborder_size = 4\n",
        false,
    ).unwrap();
    assert_eq!(cfg.general.name, "Test");
    assert_eq!(cfg.general.max_fps, 60);
    assert_eq!(cfg.general.border_size, 4);
}

#[test]
fn loader_parses_json() {
    let loader = ConfigLoader::new();
    let json = r#"{"general":{"name":"JsonCfg","blur_enabled":false,"max_fps":90}}"#;
    let cfg = loader.load_str(json, true).unwrap();
    assert_eq!(cfg.general.name, "JsonCfg");
    assert_eq!(cfg.general.max_fps, 90);
    assert_eq!(cfg.effects.blur_enabled, false);
}

#[test]
fn loader_missing_file() {
    let loader = ConfigLoader::new().with_search_path("/nonexistent/waydri");
    let result = loader.load();
    assert!(matches!(result, Err(LoadError::NotFound)));
}

#[test]
fn malformed_json_errors() {
    let loader = ConfigLoader::new();
    let result = loader.load_str("{not json", true);
    assert!(result.is_err());
}

#[test]
fn flat_config_flatten() {
    use waydri_core::config::parser::FlatConfig;
    let flat = FlatConfig::from_toml(
        "[animations]\nenabled = true\n[general]\nmax_fps = 90\n",
    );
    assert_eq!(flat.get("animations.enabled"), Some("true"));
    assert_eq!(flat.get("general.max_fps"), Some("90"));
}

#[test]
fn schema_validation() {
    use waydri_core::config::schema::{ConfigSchema, SchemaSection};
    assert!(ConfigSchema::validate(SchemaSection::General, "max_fps", "120").is_ok());
    assert!(ConfigSchema::validate(SchemaSection::General, "max_fps", "fast").is_err());
    assert!(ConfigSchema::validate(SchemaSection::General, "blur_enabled", "true").is_ok());
    assert!(ConfigSchema::validate(SchemaSection::General, "log_level", "nope").is_err());
}