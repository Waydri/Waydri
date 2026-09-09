use crate::renderer::framebuffer::Frame;
use crate::renderer::rounded::apply_rounded_corners;

use super::Effect;

#[derive(Debug, Clone)]
pub struct RoundedEffect {
    pub enabled: bool,
    pub radius: i32,
}

impl Default for RoundedEffect {
    fn default() -> Self {
        RoundedEffect {
            enabled: true,
            radius: 8,
        }
    }
}

impl Effect for RoundedEffect {
    fn enabled(&self) -> bool {
        self.enabled && self.radius > 0
    }

    fn apply(&self, frame: &mut Frame) {
        if !self.enabled() {
            return;
        }
        apply_rounded_corners(
            &mut frame.pixels,
            frame.width,
            frame.height,
            self.radius as usize,
        );
    }
}

pub fn describe() -> &'static str {
    "rounded corner masking for the output frame"
}