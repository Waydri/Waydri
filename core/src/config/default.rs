use crate::animation::Curve;
use crate::utils::{Color, Rect};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub general: GeneralConfig,
    pub output: OutputConfig,
    pub input: InputConfig,
    pub layout: LayoutConfig,
    pub animation: AnimationConfig,
    pub effects: EffectsConfig,
    pub theme: ThemeConfig,
    pub ipc: IpcConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig::default(),
            output: OutputConfig::default(),
            input: InputConfig::default(),
            layout: LayoutConfig::default(),
            animation: AnimationConfig::default(),
            effects: EffectsConfig::default(),
            theme: ThemeConfig::default(),
            ipc: IpcConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneralConfig {
    pub name: String,
    pub startup_cmd: Vec<String>,
    pub max_fps: u32,
    pub border_size: i32,
    pub gaps: i32,
    pub log_level: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            name: "Waydri".to_string(),
            startup_cmd: Vec::new(),
            max_fps: 120,
            border_size: 2,
            gaps: 8,
            log_level: "info".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputConfig {
    pub headless: bool,
    pub vsync: bool,
    pub scale: f32,
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            headless: false,
            vsync: true,
            scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputConfig {
    pub mouse_acceleration: bool,
    pub natural_scroll: bool,
    pub middle_click_to_close: bool,
    pub touchpad_scroll_factor: f32,
}

impl Default for InputConfig {
    fn default() -> Self {
        InputConfig {
            mouse_acceleration: false,
            natural_scroll: false,
            middle_click_to_close: true,
            touchpad_scroll_factor: 0.5,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutConfig {
    pub default: String,
    pub gap: i32,
    pub outer_gap: i32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            default: "master_stack".to_string(),
            gap: 8,
            outer_gap: 16,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub default_curve: Curve,
    pub window_open_duration_ms: u64,
    pub window_close_duration_ms: u64,
    pub workspace_switch_duration_ms: u64,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        AnimationConfig {
            enabled: true,
            default_curve: Curve::EaseInOut,
            window_open_duration_ms: 200,
            window_close_duration_ms: 150,
            workspace_switch_duration_ms: 250,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectsConfig {
    pub blur_enabled: bool,
    pub blur_radius: f32,
    pub shadows_enabled: bool,
    pub shadow_opacity: f32,
    pub rounded_corners_enabled: bool,
    pub corner_radius: i32,
    pub dim_inactive: f32,
    pub inactive_opacity: f32,
}

impl Default for EffectsConfig {
    fn default() -> Self {
        EffectsConfig {
            blur_enabled: true,
            blur_radius: 8.0,
            shadows_enabled: true,
            shadow_opacity: 0.6,
            rounded_corners_enabled: true,
            corner_radius: 8,
            dim_inactive: 0.0,
            inactive_opacity: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeConfig {
    pub name: String,
    pub colors: HashMap<String, Color>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert("bg".to_string(), Color::from_hex(0x11111B));
        colors.insert("fg".to_string(), Color::from_hex(0xCDD6F4));
        colors.insert("accent".to_string(), Color::from_hex(0xCBA6F7));
        ThemeConfig {
            name: "waydri".to_string(),
            colors,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpcConfig {
    pub enabled: bool,
    pub socket_path: String,
}

impl Default for IpcConfig {
    fn default() -> Self {
        IpcConfig {
            enabled: true,
            socket_path: "/tmp/waydri.sock".to_string(),
        }
    }
}

impl Config {
    pub fn keyboard_rect(&self) -> Rect {
        Rect::default()
    }
}