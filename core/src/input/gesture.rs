use crate::utils::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureKind {
    Tap,
    DoubleTap,
    LongPress,
    Swipe,
    Pinch,
    Pan,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureEvent {
    pub kind: GestureKind,
    pub position: Vec2,
    pub velocity: Vec2,
    pub scale: f32,
    pub fingers: u32,
    pub finished: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureParams {
    pub tap_timeout: f64,
    pub tap_travel: f32,
    pub long_press_timeout: f64,
    pub swipe_min_velocity: f32,
    pub double_tap_timeout: f64,
}

impl Default for GestureParams {
    fn default() -> Self {
        GestureParams {
            tap_timeout: 0.3,
            tap_travel: 24.0,
            long_press_timeout: 0.6,
            swipe_min_velocity: 240.0,
            double_tap_timeout: 0.35,
        }
    }
}

#[derive(Debug)]
pub struct GestureRecognizer {
    pub params: GestureParams,
    pub touch_start: Option<(Vec2, f64, u32)>,
    pub last_tap: Option<(Vec2, f64)>,
    pub tap_count: u32,
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

impl GestureRecognizer {
    pub fn new() -> Self {
        GestureRecognizer {
            params: GestureParams::default(),
            touch_start: None,
            last_tap: None,
            tap_count: 0,
        }
    }

    pub fn on_touch_down(&mut self, position: Vec2, time: f64, fingers: u32) -> GestureEvent {
        self.touch_start = Some((position, time, fingers));
        GestureEvent {
            kind: GestureKind::Pan,
            position,
            velocity: Vec2::ZERO,
            scale: 1.0,
            fingers,
            finished: false,
        }
    }

    pub fn on_touch_up(&mut self, position: Vec2, time: f64) -> GestureEvent {
        let fingers = self
            .touch_start
            .map(|(_, _, fingers)| fingers)
            .unwrap_or(1);
        if let Some((start, start_time, _)) = self.touch_start {
            let duration = time - start_time;
            let travel = position.distance(&start);
            if duration < self.params.tap_timeout && travel < self.params.tap_travel {
                let is_double = if let Some((tap_pos, tap_time)) = self.last_tap {
                    time - tap_time < self.params.double_tap_timeout
                        && position.distance(&tap_pos) < 60.0
                } else {
                    false
                };
                if is_double {
                    self.tap_count += 1;
                    self.last_tap = None;
                    self.touch_start = None;
                    return GestureEvent {
                        kind: GestureKind::DoubleTap,
                        position,
                        velocity: Vec2::ZERO,
                        scale: 1.0,
                        fingers,
                        finished: true,
                    };
                }
                self.tap_count = 1;
                self.last_tap = Some((position, time));
                self.touch_start = None;
                return GestureEvent {
                    kind: GestureKind::Tap,
                    position,
                    velocity: Vec2::ZERO,
                    scale: 1.0,
                    fingers,
                    finished: true,
                };
            }
            if duration >= self.params.long_press_timeout && travel < self.params.tap_travel {
                self.touch_start = None;
                return GestureEvent {
                    kind: GestureKind::LongPress,
                    position,
                    velocity: Vec2::ZERO,
                    scale: 1.0,
                    fingers,
                    finished: true,
                };
            }
            let dx = position.x - start.x;
            let dy = position.y - start.y;
            let velocity = if duration > 0.0 {
                Vec2::new(
                    dx as f32 / duration as f32 * 60.0,
                    dy as f32 / duration as f32 * 60.0,
                )
                    .normalized()
                    * self.params.swipe_min_velocity
            } else {
                Vec2::ZERO
            };
            self.touch_start = None;
            return GestureEvent {
                kind: GestureKind::Swipe,
                position,
                velocity,
                scale: 1.0,
                fingers,
                finished: true,
            };
        }
        GestureEvent {
            kind: GestureKind::Pan,
            position,
            velocity: Vec2::ZERO,
            scale: 1.0,
            fingers,
            finished: false,
        }
    }
}