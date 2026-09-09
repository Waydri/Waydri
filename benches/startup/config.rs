use std::time::Instant;

use waydri_core::config::{Config, ConfigLoader};
use waydri_core::config::parser::FlatConfig;

fn main() {
    let source = r#"
[general]
name = "waydri"
max_fps = 120
border_size = 2
gaps = 8
log_level = "info"
[animation]
enabled = true
window_open_ms = 200
[input]
natural_scroll = false
[output]
scale = 1.0
"#;
    let loader = ConfigLoader::new();
    let start = Instant::now();
    for _ in 0..100_000 {
        let config = loader.load_str(source, false).unwrap();
        assert_eq!(config.general.border_size, 2);
    }
    let elapsed = start.elapsed();
    println!("toml parse: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for _ in 0..100_000 {
        let flat = FlatConfig::from_toml(source);
        assert_eq!(flat.get("general.max_fps"), Some("120"));
    }
    let elapsed = start.elapsed();
    println!("flat parse: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for _ in 0..100_000_000 / 1000 {
        let config = Config::default();
        assert_eq!(config.general.name, "Waydri");
    }
    let elapsed = start.elapsed();
    println!("default build: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
}