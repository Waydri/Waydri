use std::time::Duration;

use crate::animation::{Curve, Transition};

fn make(from: f32, to: f32, millis: u64, curve: Curve) -> Transition {
    Transition::new(from, to, Duration::from_millis(millis)).with_curve(curve)
}

#[derive(Debug)]
pub struct EffectAnimator {
    pub window_open: Transition,
    pub window_close: Transition,
    pub workspace_switch: Transition,
    pub opacity: Transition,
    pub scale: Transition,
}

impl Default for EffectAnimator {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectAnimator {
    pub fn new() -> Self {
        EffectAnimator {
            window_open: make(0.0, 1.0, 200, Curve::EaseOut),
            window_close: make(1.0, 0.0, 150, Curve::EaseIn),
            workspace_switch: make(0.0, 1.0, 250, Curve::EaseInOut),
            opacity: make(1.0, 1.0, 120, Curve::Linear),
            scale: make(0.9, 1.0, 200, Curve::EaseOut),
        }
    }

    pub fn open_window(&mut self) {
        self.window_open = make(0.0, 1.0, 200, Curve::EaseOut);
        self.opacity = make(0.0, 1.0, 200, Curve::EaseOut);
        self.scale = make(0.9, 1.0, 200, Curve::EaseOut);
    }

    pub fn close_window(&mut self) {
        self.window_close = make(1.0, 0.0, 150, Curve::EaseIn);
    }

    pub fn switch_workspace(&mut self) {
        self.workspace_switch = make(0.0, 1.0, 250, Curve::EaseInOut);
    }

    pub fn tick(&mut self, delta: Duration) {
        self.window_open.tick(delta);
        self.window_close.tick(delta);
        self.workspace_switch.tick(delta);
        self.opacity.tick(delta);
        self.scale.tick(delta);
    }

    pub fn is_animating(&self) -> bool {
        !self.window_open.finished()
            || !self.window_close.finished()
            || !self.workspace_switch.finished()
            || !self.opacity.finished()
            || !self.scale.finished()
    }
}