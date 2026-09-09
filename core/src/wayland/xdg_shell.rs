use crate::utils::Rect;
use crate::window::WindowId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgSurfaceKind {
    Toplevel,
    Popup,
    None,
}

#[derive(Debug, Clone)]
pub struct XdgSurface {
    pub id: u32,
    pub surface: WindowId,
    pub kind: XdgSurfaceKind,
}

impl XdgSurface {
    pub fn new(id: u32, surface: WindowId) -> Self {
        XdgSurface {
            id,
            surface,
            kind: XdgSurfaceKind::None,
        }
    }

    pub fn set_toplevel(&mut self) {
        self.kind = XdgSurfaceKind::Toplevel;
    }

    pub fn set_popup(&mut self) {
        self.kind = XdgSurfaceKind::Popup;
    }
}

#[derive(Debug, Clone)]
pub struct XdgToplevel {
    pub surface: WindowId,
    pub title: String,
    pub app_id: String,
    pub parent: Option<WindowId>,
    pub min_size: (i32, i32),
    pub max_size: (i32, i32),
    pub geometry: Rect,
    pub maximized: bool,
    pub fullscreen: bool,
    pub activated: bool,
}

impl XdgToplevel {
    pub fn new(surface: WindowId) -> Self {
        XdgToplevel {
            surface,
            title: String::new(),
            app_id: String::new(),
            parent: None,
            min_size: (0, 0),
            max_size: (0, 0),
            geometry: Rect::default(),
            maximized: false,
            fullscreen: false,
            activated: false,
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_app_id(&mut self, app_id: String) {
        self.app_id = app_id;
    }

    pub fn set_min_size(&mut self, x: i32, y: i32) {
        self.min_size = (x, y);
    }

    pub fn set_max_size(&mut self, x: i32, y: i32) {
        self.max_size = (x, y);
    }

    pub fn effective_min(&self) -> (i32, i32) {
        (self.min_size.0.max(1), self.min_size.1.max(1))
    }
}