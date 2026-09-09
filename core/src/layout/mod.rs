pub mod custom;
pub mod dwindle;
pub mod dynamic;
pub mod grid;
pub mod master_stack;

use std::collections::HashMap;

use crate::utils::Rect;
use crate::window::WindowId;

pub trait Layout: Send + Sync {
    fn name(&self) -> &'static str;
    fn arrange(&self, area: Rect, windows: &[WindowId]) -> Vec<Rect>;
}

pub struct LayoutManager {
    pub layouts: HashMap<String, Box<dyn Layout>>,
    pub current: String,
    pub gap: i32,
}

impl LayoutManager {
    pub fn new() -> Self {
        let mut layouts: HashMap<String, Box<dyn Layout>> = HashMap::new();
        let master = master_stack::MasterStack::default();
        let dwindle = dwindle::Dwindle::default();
        let grid = grid::Grid::default();
        let custom = custom::CustomLayout::default();
        let dynamic = dynamic::DynamicLayout::default();
        layouts.insert(master.name().to_string(), Box::new(master));
        layouts.insert(dwindle.name().to_string(), Box::new(dwindle));
        layouts.insert(grid.name().to_string(), Box::new(grid));
        layouts.insert(custom.name().to_string(), Box::new(custom));
        layouts.insert(dynamic.name().to_string(), Box::new(dynamic));
        LayoutManager {
            layouts,
            current: "master_stack".to_string(),
            gap: 8,
        }
    }

    pub fn with_default() -> Self {
        LayoutManager::new()
    }

    pub fn set_layout(&mut self, name: &str) -> bool {
        if self.layouts.contains_key(name) {
            self.current = name.to_string();
            return true;
        }
        false
    }

    pub fn arrange(&self, name: &str, area: Rect, windows: &[WindowId]) -> Vec<Rect> {
        if let Some(layout) = self.layouts.get(name) {
            layout.arrange(area, windows)
        } else if let Some(fallback) = self.layouts.get(&self.current) {
            fallback.arrange(area, windows)
        } else {
            windows.iter().map(|_| area).collect()
        }
    }

    pub fn current_name(&self) -> &str {
        &self.current
    }

    pub fn available(&self) -> Vec<&'static str> {
        self.layouts.values().map(|layout| layout.name()).collect()
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}