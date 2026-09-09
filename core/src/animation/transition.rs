use std::time::Duration;

use super::{AnimationState, Curve};

pub struct Transition {
    pub from: f32,
    pub to: f32,
    pub duration: Duration,
    pub curve: Curve,
    pub elapsed: Duration,
    pub state: AnimationState,
    pub on_finish: Option<Box<dyn FnOnce() + Send>>,
}

impl Transition {
    pub fn new(from: f32, to: f32, duration: Duration) -> Self {
        Transition {
            from,
            to,
            duration,
            curve: Curve::default(),
            elapsed: Duration::ZERO,
            state: AnimationState::Running,
            on_finish: None,
        }
    }

    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    pub fn on_finish<F: FnOnce() + Send + 'static>(mut self, f: F) -> Self {
        self.on_finish = Some(Box::new(f));
        self
    }

    pub fn tick(&mut self, dt: Duration) {
        if self.state != AnimationState::Running {
            return;
        }
        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.state = AnimationState::Finished;
            if let Some(f) = self.on_finish.take() {
                f();
            }
        }
    }

    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Running;
        }
    }

    pub fn progress(&self) -> f32 {
        if self.duration.is_zero() {
            return 1.0;
        }
        (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
    }

    pub fn current_value(&self) -> f32 {
        let t = self.curve.apply(self.progress());
        self.from + (self.to - self.from) * t
    }

    pub fn finished(&self) -> bool {
        self.state == AnimationState::Finished
    }
}

impl std::fmt::Debug for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transition")
            .field("from", &self.from)
            .field("to", &self.to)
            .field("duration", &self.duration)
            .field("curve", &self.curve)
            .field("elapsed", &self.elapsed)
            .field("state", &self.state)
            .field("on_finish", &self.on_finish.is_some())
            .finish()
    }
}
