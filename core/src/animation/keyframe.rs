use std::time::Duration;

use super::Curve;

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub time: Duration,
    pub value: f32,
    pub curve: Curve,
}

impl Keyframe {
    pub fn new(time: Duration, value: f32) -> Self {
        Keyframe {
            time,
            value,
            curve: Curve::default(),
        }
    }

    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = curve;
        self
    }

    pub fn interpolate(&self, next: &Keyframe, elapsed: Duration) -> f32 {
        let total = next.time.saturating_sub(self.time);
        if total.is_zero() {
            return self.value;
        }
        let local = elapsed.saturating_sub(self.time);
        let t = local.as_secs_f32() / total.as_secs_f32();
        let eased = self.curve.apply(t);
        self.value + (next.value - self.value) * eased
    }
}
