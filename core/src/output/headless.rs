use crate::renderer::{Frame, SoftwareRenderer};
use crate::utils::Color;

use super::OutputMode;

#[derive(Debug, Clone)]
pub struct HeadlessBuffer {
    pub width: u32,
    pub height: u32,
    pub mode: OutputMode,
    pub frame: usize,
}

impl HeadlessBuffer {
    pub fn new(mode: OutputMode) -> Self {
        HeadlessBuffer {
            width: mode.width,
            height: mode.height,
            mode,
            frame: 0,
        }
    }

    pub fn present(&mut self, renderer: &mut SoftwareRenderer) {
        if renderer.frame.width as u32 != self.width || renderer.frame.height as u32 != self.height {
            renderer.resize(self.width as usize, self.height as usize);
        }
        self.frame += 1;
    }

    pub fn generate_test_frame(&self) -> Frame {
        let c = Color::new(0.1, 0.1, 0.16, 1.0);
        Frame::new(self.width as usize, self.height as usize).blank()
            .clear_c(c)
    }
}

trait FrameFactory {
    fn blank(self) -> Self;
    fn clear_c(self, color: Color) -> Self;
}

impl FrameFactory for Frame {
    fn blank(self) -> Self {
        self
    }

    fn clear_c(mut self, color: Color) -> Self {
        self.clear_with(color);
        self
    }
}

pub fn headless_outputs() -> Vec<&'static str> {
    vec!["headless-0", "headless-1"]
}