use crate::renderer::framebuffer::Frame;
use crate::renderer::opacity::multiply_alpha;

use super::Effect;

#[derive(Debug, Clone)]
pub struct OpacityEffect {
    pub enabled: bool,
    pub alpha: f32,
    pub target_alpha: f32,
}

impl Default for OpacityEffect {
    fn default() -> Self {
        OpacityEffect {
            enabled: true,
            alpha: 1.0,
            target_alpha: 1.0,
        }
    }
}

impl OpacityEffect {
    pub fn step(&mut self, step: f32) {
        let difference = self.target_alpha - self.alpha;
        if difference.abs() > step {
            self.alpha += difference.signum() * step;
        } else {
            self.alpha = self.target_alpha;
        }
    }
}

impl Effect for OpacityEffect {
    fn enabled(&self) -> bool {
        self.enabled && self.alpha < 1.0
    }

    fn apply(&self, frame: &mut Frame) {
        if !self.enabled() {
            return;
        }
        multiply_alpha(&mut frame.pixels, self.alpha);
    }
}

pub fn describe() -> &'static str {
    "uniform opacity effect over the composed frame"
}