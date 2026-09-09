use crate::utils::Rect;
use crate::window::WindowId;

use super::Layout;

pub struct CustomLayout {
    pub slots: Vec<Rect>,
    pub gap: i32,
}

impl Default for CustomLayout {
    fn default() -> Self {
        CustomLayout {
            slots: Vec::new(),
            gap: 8,
        }
    }
}

impl CustomLayout {
    pub fn with_slots(mut self, slots: Vec<Rect>) -> Self {
        self.slots = slots;
        self
    }

    pub fn ratio_slots(area: Rect, ratios: &[f32]) -> Vec<Rect> {
        let total: f32 = ratios.iter().sum();
        let total = if total <= 0.0 { 1.0 } else { total };
        let mut slots = Vec::new();
        let mut offset = area.x;
        for ratio in ratios {
            let width = ((area.width as f32 * (ratio / total)) as i32).max(1);
            slots.push(Rect::new(offset, area.y, width, area.height));
            offset += width;
        }
        slots
    }
}

impl Layout for CustomLayout {
    fn name(&self) -> &'static str {
        "custom"
    }

    fn arrange(&self, area: Rect, windows: &[WindowId]) -> Vec<Rect> {
        if windows.is_empty() {
            return Vec::new();
        }
        if self.slots.is_empty() {
            return vec![area.grow(-self.gap)];
        }
        let slots = self.slots.clone();
        let mut result = Vec::with_capacity(windows.len());
        let count = windows.len();
        let slot_count = slots.len();
        for index in 0..count {
            let mut rect = slots[index % slot_count];
            if index >= slot_count {
                rect = slots[slot_count - 1];
            }
            result.push(rect);
        }
        result
    }
}

pub fn describe() -> &'static str {
    "custom layout driven by a user defined slot set"
}