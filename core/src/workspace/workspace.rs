use crate::window::WindowId;

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: u64,
    pub name: String,
    pub layout_name: String,
    pub windows: Vec<WindowId>,
    pub visible: bool,
}

impl Workspace {
    pub fn new(id: u64, name: String) -> Self {
        Workspace {
            id,
            name,
            layout_name: "master_stack".to_string(),
            windows: Vec::new(),
            visible: true,
        }
    }

    pub fn with_layout(mut self, layout: &str) -> Self {
        self.layout_name = layout.to_string();
        self
    }

    pub fn set_layout(&mut self, layout: &str) {
        self.layout_name = layout.to_string();
    }

    pub fn add_window(&mut self, window: WindowId) {
        if !self.windows.contains(&window) {
            self.windows.push(window);
        }
    }

    pub fn remove_window(&mut self, window: WindowId) -> bool {
        let before = self.windows.len();
        self.windows.retain(|w| *w != window);
        self.windows.len() != before
    }

    pub fn has_window(&self, window: WindowId) -> bool {
        self.windows.contains(&window)
    }

    pub fn window_ids(&self) -> Vec<WindowId> {
        self.windows.clone()
    }

    pub fn layout_name(&self) -> &str {
        &self.layout_name
    }

    pub fn window_count(&self) -> usize {
        self.windows.len()
    }
}