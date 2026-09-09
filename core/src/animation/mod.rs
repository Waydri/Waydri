pub mod curve;
pub mod keyframe;
pub mod spring;
pub mod transition;

pub use curve::Curve;
pub use keyframe::Keyframe;
pub use spring::Spring;
pub use transition::Transition;

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    Idle,
    Running,
    Paused,
    Finished,
}

pub trait Animatable: Send + Sync {
    fn animate(&mut self, progress: f32);
    fn finished(&self) -> bool;
    fn duration(&self) -> Duration;
}

pub fn eased(t: f32, curve: Curve) -> f32 {
    curve.apply(t.clamp(0.0, 1.0))
}
