use crate::renderer::framebuffer::Frame;
use crate::renderer::shadow::{ShadowSpec, shadow_via_blur};
use crate::utils::{Color, Rect};

use super::Effect;

#[derive(Debug, Clone)]
pub struct ShadowEffect {
    pub enabled: bool,
    pub blur_radius: f32,
    pub opacity: f32,
    pub offset: (f32, f32),
    pub color: Color,
}

impl Default for ShadowEffect {
    fn default() -> Self {
        ShadowEffect {
            enabled: true,
            blur_radius: 12.0,
            opacity: 0.6,
            offset: (0.0, 4.0),
            color: Color::BLACK,
        }
    }
}

impl ShadowEffect {
    pub fn spec(&self) -> ShadowSpec {
        ShadowSpec {
            blur_radius: self.blur_radius,
            opacity: self.opacity,
            offset: self.offset,
            color: self.color,
        }
    }
}

impl Effect for ShadowEffect {
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn apply(&self, frame: &mut Frame) {
        if !self.enabled {
            return;
        }
        let target = Rect::new(
            frame.width as i32 / 4,
            frame.height as i32 / 4,
            frame.width as i32 / 2,
            frame.height as i32 / 2,
        );
        shadow_via_blur(
            &mut frame.pixels,
            frame.width,
            frame.height,
            &self.spec(),
            target,
        );
    }
}

pub fn describe() -> &'static str {
    "soft drop shadow effect around window rectangles"
}