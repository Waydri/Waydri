use crate::utils::Rect;
use crate::window::WindowId;

use super::Layout;

pub struct MasterStack {
    pub ratio: f32,
    pub gap: i32,
    pub outer_gap: i32,
}

impl Default for MasterStack {
    fn default() -> Self {
        MasterStack {
            ratio: 0.6,
            gap: 8,
            outer_gap: 16,
        }
    }
}

impl MasterStack {
    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.1, 0.9);
        self
    }
}

impl Layout for MasterStack {
    fn name(&self) -> &'static str {
        "master_stack"
    }

    fn arrange(&self, area: Rect, windows: &[WindowId]) -> Vec<Rect> {
        if windows.is_empty() {
            return Vec::new();
        }
        if windows.len() == 1 {
            return vec![area.grow(-self.outer_gap).shrink(0)];
        }
        let inner = area.grow(-self.outer_gap);
        let master_width = (inner.width as f32 * self.ratio) as i32;
        let master = Rect::new(inner.x, inner.y, master_width, inner.height);
        let stack_x = inner.x + master_width + self.gap;
        let stack_width = (inner.width - master_width - self.gap).max(1);
        let mut result = Vec::with_capacity(windows.len());
        result.push(master);
        let stack_count = windows.len() - 1;
        let stack_height = (inner.height - self.gap * (stack_count as i32 - 1)) / stack_count.max(1) as i32;
        for index in 0..stack_count {
            let y = inner.y + index as i32 * (stack_height + self.gap);
            result.push(Rect::new(stack_x, y, stack_width, stack_height));
        }
        result
    }
}

pub fn describe() -> &'static str {
    "master stack layout with a master pane and a stacked column"
}