use crate::renderer::gradient::{LinearGradient, fill_gradient};
use crate::renderer::framebuffer::Frame;
use crate::utils::Color;

use super::Effect;

#[derive(Debug, Clone)]
pub struct GradientEffect {
    pub enabled: bool,
    pub angle: f32,
    pub color1: Color,
    pub color2: Color,
}

impl Default for GradientEffect {
    fn default() -> Self {
        GradientEffect {
            enabled: false,
            angle: 45.0,
            color1: Color::from_hex(0x1E1E28),
            color2: Color::from_hex(0x2E2E38),
        }
    }
}

impl GradientEffect {
    pub fn two_colors(color1: Color, color2: Color, angle: f32) -> Self {
        GradientEffect {
            enabled: true,
            angle,
            color1,
            color2,
        }
    }
}

impl Effect for GradientEffect {
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn apply(&self, frame: &mut Frame) {
        if !self.enabled {
            return;
        }
        let gradient = LinearGradient::two(self.color1, self.color2);
        fill_gradient(
            &mut frame.pixels,
            frame.width,
            frame.height,
            &gradient,
            self.angle,
        );
    }
}

pub fn describe() -> &'static str {
    "linear gradient effect layered beneath the window stack"
}