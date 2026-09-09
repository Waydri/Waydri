use crate::utils::{Color, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub u64);

#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub placement: Rect,
    pub alpha: f32,
    pub visible: bool,
    pub floating: bool,
    pub fullscreen: bool,
    pub workspace: u64,
    pub buffer: Option<Vec<u8>>,
    pub buffer_size: (usize, usize),
    pub color: Color,
    pub depth: u64,
}

impl Window {
    pub fn new(id: WindowId, title: String, app_id: String, workspace: u64) -> Self {
        Window {
            id,
            title,
            app_id,
            placement: Rect::default(),
            alpha: 1.0,
            visible: true,
            floating: false,
            fullscreen: false,
            workspace,
            buffer: None,
            buffer_size: (0, 0),
            color: Color::from_hex(0x1E1E28),
            depth: 0,
        }
    }

    pub fn set_buffer(&mut self, buffer: Vec<u8>, width: usize, height: usize) {
        self.buffer_size = (width, height);
        self.buffer = Some(buffer);
    }

    pub fn surface_size(&self) -> (usize, usize) {
        self.buffer_size
    }

    pub fn center(&self) -> (i32, i32) {
        let c = self.placement.center();
        (c.x as i32, c.y as i32)
    }

    pub fn intersects(&self, rect: Rect) -> bool {
        self.placement.intersects(&rect)
    }
}