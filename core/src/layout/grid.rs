use crate::utils::Rect;
use crate::window::WindowId;

use super::Layout;

pub struct Grid {
    pub columns: usize,
    pub gap: i32,
    pub outer_gap: i32,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            columns: 2,
            gap: 8,
            outer_gap: 16,
        }
    }
}

impl Layout for Grid {
    fn name(&self) -> &'static str {
        "grid"
    }

    fn arrange(&self, area: Rect, windows: &[WindowId]) -> Vec<Rect> {
        if windows.is_empty() {
            return Vec::new();
        }
        if windows.len() == 1 {
            return vec![area.grow(-self.outer_gap)];
        }
        let count = windows.len();
        let columns = self.columns.clamp(1, count.max(1));
        let rows = (count + columns - 1) / columns;
        let inner = area.grow(-self.outer_gap);
        let cell_width = (inner.width - self.gap * (columns as i32 - 1)) / columns as i32;
        let cell_height = (inner.height - self.gap * (rows as i32 - 1)) / rows as i32;
        let mut result = Vec::with_capacity(count);
        for index in 0..count {
            let row = index / columns;
            let col = index % columns;
            result.push(Rect::new(
                inner.x + col as i32 * (cell_width + self.gap),
                inner.y + row as i32 * (cell_height + self.gap),
                cell_width,
                cell_height,
            ));
        }
        result
    }
}

pub fn describe() -> &'static str {
    "grid layout with equally sized cells"
}