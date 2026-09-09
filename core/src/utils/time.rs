use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Clock {
    start: Instant,
    last: Instant,
    frame_count: u64,
    fps_accumulator: f32,
    fps: f32,
}

impl Clock {
    pub fn new() -> Self {
        let now = Instant::now();
        Clock {
            start: now,
            last: now,
            frame_count: 0,
            fps_accumulator: 0.0,
            fps: 0.0,
        }
    }

    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now.duration_since(self.last);
        self.last = now;
        self.frame_count += 1;
        self.fps_accumulator += delta.as_secs_f32();
        if self.fps_accumulator >= 1.0 {
            self.fps = self.frame_count as f32 / self.fps_accumulator;
            self.frame_count = 0;
            self.fps_accumulator = 0.0;
        }
        delta
    }

    pub fn elapsed(&self) -> Duration {
        self.last.duration_since(self.start)
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn uptime_secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

pub fn now() -> Instant {
    Instant::now()
}

pub fn now_millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

pub fn duration_millis(ms: u64) -> Duration {
    Duration::from_millis(ms)
}
