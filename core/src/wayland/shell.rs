use crate::utils::Rect;
use crate::window::WindowId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellKind {
    ShellKindWl,
    Xdg,
    Layer,
    Popup,
}

pub struct ShellSurface {
    pub surface: WindowId,
    pub kind: ShellKind,
    pub title: String,
    pub geometry: Rect,
}

impl ShellSurface {
    pub fn new(surface: WindowId, kind: ShellKind, title: String) -> Self {
        ShellSurface {
            surface,
            kind,
            title,
            geometry: Rect::default(),
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_geometry(&mut self, geometry: Rect) {
        self.geometry = geometry;
    }
}

pub fn scroll_behavior(axis: u32) -> &'static str {
    match axis {
        0 => "vertical",
        1 => "horizontal",
        _ => "diagonal",
    }
}