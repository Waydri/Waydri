use crate::renderer::blur;
use crate::renderer::framebuffer::Frame;

use super::Effect;

#[derive(Debug, Clone)]
pub struct BlurEffect {
    pub enabled: bool,
    pub radius: f32,
}

impl Default for BlurEffect {
    fn default() -> Self {
        BlurEffect {
            enabled: true,
            radius: 8.0,
        }
    }
}

impl BlurEffect {
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius.max(0.0);
        self
    }
}

impl Effect for BlurEffect {
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn apply(&self, frame: &mut Frame) {
        if !self.enabled || self.radius <= 0.0 {
            return;
        }
        let radius = self.radius.round() as usize;
        if radius == 0 {
            return;
        }
        blur::blur(&mut frame.pixels, frame.width, frame.height, radius);
    }
}

pub fn describe() -> &'static str {
    "gaussian blur effect applied to the output frame"
}