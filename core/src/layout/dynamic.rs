use crate::utils::Rect;
use crate::window::WindowId;

use super::{Layout, master_stack::MasterStack, grid::Grid};

pub struct DynamicLayout {
    pub master: MasterStack,
    pub grid: Grid,
    pub threshold: usize,
}

impl Default for DynamicLayout {
    fn default() -> Self {
        DynamicLayout {
            master: MasterStack::default(),
            grid: Grid::default(),
            threshold: 6,
        }
    }
}

impl Layout for DynamicLayout {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn arrange(&self, area: Rect, windows: &[WindowId]) -> Vec<Rect> {
        if windows.len() > self.threshold {
            self.grid.arrange(area, windows)
        } else {
            self.master.arrange(area, windows)
        }
    }
}

pub fn describe() -> &'static str {
    "dynamic layout that switches between master stack and grid by window count"
}