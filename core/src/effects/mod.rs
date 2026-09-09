pub mod animation;
pub mod blur;
pub mod gradient;
pub mod opacity;
pub mod rounded;
pub mod shadow;

use crate::renderer::framebuffer::Frame;

use blur::BlurEffect;
use gradient::GradientEffect;
use opacity::OpacityEffect;
use rounded::RoundedEffect;
use shadow::ShadowEffect;

#[derive(Debug)]
pub struct EffectsState {
    pub blur: BlurEffect,
    pub gradient: GradientEffect,
    pub shadow: ShadowEffect,
    pub opacity: OpacityEffect,
    pub rounded: RoundedEffect,
}

impl EffectsState {
    pub fn new() -> Self {
        EffectsState {
            blur: BlurEffect::default(),
            gradient: GradientEffect::default(),
            shadow: ShadowEffect::default(),
            opacity: OpacityEffect::default(),
            rounded: RoundedEffect::default(),
        }
    }

    pub fn apply_all(&self, frame: &mut Frame) {
        self.opacity.apply(frame);
        self.gradient.apply(frame);
        self.blur.apply(frame);
        self.rounded.apply(frame);
    }
}

impl Default for EffectsState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Effect {
    fn enabled(&self) -> bool;
    fn apply(&self, frame: &mut Frame);
}