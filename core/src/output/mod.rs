pub mod android;
pub mod backend;
pub mod drm;
pub mod egl;
pub mod gbm;
pub mod headless;
pub mod wayland;

use crate::utils::{Rect, Transform};

pub use backend::{BackendContext, OutputManager};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Headless,
    Drm,
    Wayland,
    Egl,
    Gbm,
    Android,
}

impl BackendKind {
    pub fn name(&self) -> &'static str {
        match self {
            BackendKind::Headless => "headless",
            BackendKind::Drm => "drm",
            BackendKind::Wayland => "wayland",
            BackendKind::Egl => "egl",
            BackendKind::Gbm => "gbm",
            BackendKind::Android => "android",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputMode {
    pub width: u32,
    pub height: u32,
    pub refresh_mhz: u32,
}

impl OutputMode {
    pub const fn new(width: u32, height: u32, refresh_hz: u32) -> Self {
        OutputMode {
            width,
            height,
            refresh_mhz: refresh_hz * 1000,
        }
    }

    pub fn refresh_hz(&self) -> u32 {
        self.refresh_mhz / 1000
    }

    pub fn geometry(&self) -> Rect {
        Rect::new(0, 0, self.width as i32, self.height as i32)
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    pub id: u64,
    pub name: String,
    pub desc: String,
    pub x: i32,
    pub y: i32,
    pub scale: f32,
    pub transform: Transform,
    pub current_mode: OutputMode,
    pub modes: Vec<OutputMode>,
    pub enabled: bool,
    pub backend: BackendKind,
    pub native_fd: Option<i32>,
}

impl Output {
    pub fn headless(id: u64, name: String, width: u32, height: u32, refresh: u32) -> Self {
        let desc = format!("headless output {name}");
        Output {
            id,
            name,
            desc,
            x: 0,
            y: 0,
            scale: 1.0,
            transform: Transform::Normal,
            current_mode: OutputMode::new(width, height, refresh),
            modes: vec![
                OutputMode::new(1920, 1080, 120),
                OutputMode::new(1366, 768, 60),
                OutputMode::new(1280, 720, 60),
            ],
            enabled: true,
            backend: BackendKind::Headless,
            native_fd: None,
        }
    }

    pub fn geometry(&self) -> Rect {
        let mode = self.current_mode;
        let (w, h) = self.transform.apply(mode.width as i32, mode.height as i32);
        Rect::new(self.x, self.y, w, h)
    }

    pub fn set_mode(&mut self, mode: OutputMode) {
        self.current_mode = mode;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.geometry().contains(x, y)
    }

    pub fn pixels_per_point(&self) -> f32 {
        self.scale
    }
}