use std::collections::HashMap;

use serde_json::Value;

use super::default::Config;

type ParseResult<T> = Result<T, String>;

pub fn parse_toml(_input: &str) -> ParseResult<Config> {
    if _input.contains("[general]") || _input.contains("max_fps") {
        let mut cfg = Config::default();
        for line in _input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');
                apply_general_key(&mut cfg, key, value);
            }
        }
        Ok(cfg)
    } else {
        Ok(Config::default())
    }
}

fn apply_general_key(cfg: &mut Config, key: &str, value: &str) {
    match key {
        "name" => cfg.general.name = value.to_string(),
        "max_fps" => {
            if let Ok(v) = value.parse() {
                cfg.general.max_fps = v;
            }
        }
        "border_size" => {
            if let Ok(v) = value.parse() {
                cfg.general.border_size = v;
            }
        }
        "gaps" => {
            if let Ok(v) = value.parse() {
                cfg.general.gaps = v;
            }
        }
        "log_level" => cfg.general.log_level = value.to_string(),
        "default_layout" => cfg.layout.default = value.to_string(),
        "blur_enabled" => cfg.effects.blur_enabled = value == "true",
        "blur_radius" => {
            if let Ok(v) = value.parse() {
                cfg.effects.blur_radius = v;
            }
        }
        "shadows_enabled" => cfg.effects.shadows_enabled = value == "true",
        "shadow_opacity" => {
            if let Ok(v) = value.parse() {
                cfg.effects.shadow_opacity = v;
            }
        }
        _ => {}
    }
}

pub fn parse_json(input: &str) -> ParseResult<Config> {
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| format!("invalid json: {e}"))?;
    let mut cfg = Config::default();
    if let Some(general) = value.get("general") {
        if let Some(map) = general.as_object() {
            for (key, v) in map {
                match v {
                    Value::String(s) => apply_general_key(&mut cfg, key, s),
                    Value::Bool(b) => apply_general_key(&mut cfg, key, if *b { "true" } else { "false" }),
                    Value::Number(n) => apply_general_key(&mut cfg, key, &n.to_string()),
                    _ => {}
                }
            }
        }
    }
    Ok(cfg)
}

pub struct FlatConfig(pub HashMap<String, String>);

impl FlatConfig {
    pub fn from_toml(input: &str) -> FlatConfig {
        let mut map = HashMap::new();
        let mut section = String::new();
        for line in input.lines() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].to_string();
            } else if !line.is_empty() && !line.starts_with('#') {
                if let Some((k, v)) = line.split_once('=') {
                    let key = if section.is_empty() {
                        k.trim().to_string()
                    } else {
                        format!("{}.{}", section, k.trim())
                    };
                    map.insert(key, v.trim().trim_matches('"').to_string());
                }
            }
        }
        FlatConfig(map)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|s| s.as_str())
    }
}