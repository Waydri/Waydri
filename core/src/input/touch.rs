use crate::utils::Vec2;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchAction {
    Down,
    Move,
    Up,
    Cancel,
}

#[derive(Debug, Clone, Copy)]
pub struct TouchPoint {
    pub id: i32,
    pub position: Vec2,
    pub start_position: Vec2,
    pub start_time: f64,
    pub down: bool,
}

#[derive(Debug)]
pub struct Touch {
    pub pointers: HashMap<i32, TouchPoint>,
    pub max_pointers: usize,
    last_tap_time: f64,
    last_tap_position: Vec2,
    pub tap_count: u32,
}

impl Default for Touch {
    fn default() -> Self {
        Self::new()
    }
}

impl Touch {
    pub fn new() -> Self {
        Touch {
            pointers: HashMap::new(),
            max_pointers: 10,
            last_tap_time: 0.0,
            last_tap_position: Vec2::ZERO,
            tap_count: 0,
        }
    }

    pub fn down(&mut self, id: i32, position: Vec2, time: f64) {
        self.pointers.insert(
            id,
            TouchPoint {
                id,
                position,
                start_position: position,
                start_time: time,
                down: true,
            },
        );
    }

    pub fn move_point(&mut self, id: i32, position: Vec2) -> bool {
        if let Some(point) = self.pointers.get_mut(&id) {
            point.position = position;
            return true;
        }
        false
    }

    pub fn up(&mut self, id: i32, time: f64) -> Option<TouchPoint> {
        let point = self.pointers.remove(&id)?;
        let travel = point.position.distance(&point.start_position);
        let duration = time - point.start_time;
        if travel < 24.0 && duration < 0.35 {
            if time - self.last_tap_time < 0.35
                && self.last_tap_position.distance(&point.position) < 60.0
            {
                self.tap_count += 1;
            } else {
                self.tap_count = 1;
            }
            self.last_tap_time = time;
            self.last_tap_position = point.position;
        } else {
            self.tap_count = 0;
        }
        Some(point)
    }

    pub fn cancel(&mut self) {
        self.pointers.clear();
    }

    pub fn active_count(&self) -> usize {
        self.pointers.len()
    }

    pub fn centroid(&self) -> Option<Vec2> {
        let mut sum = Vec2::ZERO;
        let mut count = 0.0;
        for point in self.pointers.values() {
            sum.x += point.position.x;
            sum.y += point.position.y;
            count += 1.0;
        }
        if count > 0.0 {
            Some(Vec2::new(sum.x / count, sum.y / count))
        } else {
            None
        }
    }

    pub fn average_start_distance(&self) -> f32 {
        let Some(centroid) = self.centroid() else {
            return 0.0;
        };
        let count = self.pointers.len() as f32;
        if count == 0.0 {
            return 0.0;
        }
        let sum: f32 = self
            .pointers
            .values()
            .map(|point| point.position.distance(&centroid))
            .sum();
        sum / count
    }

    pub fn pinch_ratio(&self) -> f32 {
        let Some(centroid) = self.centroid() else {
            return 1.0;
        };
        if self.pointers.len() < 2 {
            return 1.0;
        }
        let start: f32 = self
            .pointers
            .values()
            .map(|point| point.start_position.distance(&centroid))
            .sum::<f32>()
            / 2.0;
        let current: f32 = self
            .pointers
            .values()
            .map(|point| point.position.distance(&centroid))
            .sum::<f32>()
            / 2.0;
        if start <= 0.0 {
            1.0
        } else {
            current / start
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TouchEvent {
    pub id: i32,
    pub action: TouchAction,
    pub position: Vec2,
}