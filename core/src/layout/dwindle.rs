use crate::utils::Rect;
use crate::window::WindowId;

use super::Layout;

pub struct Dwindle {
    pub gap: i32,
    pub outer_gap: i32,
    pub split_ratio: f32,
}

impl Default for Dwindle {
    fn default() -> Self {
        Dwindle {
            gap: 8,
            outer_gap: 16,
            split_ratio: 0.5,
        }
    }
}

impl Layout for Dwindle {
    fn name(&self) -> &'static str {
        "dwindle"
    }

    fn arrange(&self, area: Rect, windows: &[WindowId]) -> Vec<Rect> {
        if windows.is_empty() {
            return Vec::new();
        }
        let inner = area.grow(-self.outer_gap);
        if windows.len() == 1 {
            return vec![inner];
        }
        let mut rects = Vec::with_capacity(windows.len());
        let mut pending: Vec<Rect> = vec![inner];
        let mut vertical = true;
        for index in 0..windows.len() {
            let Some(current) = pending.pop() else {
                rects.push(inner);
                continue;
            };
            rects.push(current);
            let remaining = windows.len() - index - 1;
            if remaining == 0 {
                break;
            }
            let (first, second) = if vertical {
                let split = (current.width as f32 * self.split_ratio) as i32;
                (
                    Rect::new(current.x, current.y, split.max(1), current.height),
                    Rect::new(
                        current.x + split + self.gap,
                        current.y,
                        (current.width - split - self.gap).max(1),
                        current.height,
                    ),
                )
            } else {
                let split = (current.height as f32 * self.split_ratio) as i32;
                (
                    Rect::new(current.x, current.y, current.width, split.max(1)),
                    Rect::new(
                        current.x,
                        current.y + split + self.gap,
                        current.width,
                        (current.height - split - self.gap).max(1),
                    ),
                )
            };
            if remaining >= 2 {
                pending.push(second);
            } else {
                let _ = second;
            }
            if remaining >= 1 {
                pending.push(first);
            }
            vertical = !vertical;
        }
        rects
    }
}

pub fn describe() -> &'static str {
    "dwindle layout with an alternating spiral split"
}