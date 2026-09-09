use crate::window::WindowId;

#[derive(Debug, Clone)]
pub struct SpecialWorkspace {
    pub id: u64,
    pub name: String,
    pub windows: Vec<WindowId>,
    pub visible: bool,
}

impl SpecialWorkspace {
    pub fn new(id: u64, name: String) -> Self {
        SpecialWorkspace {
            id,
            name,
            windows: Vec::new(),
            visible: false,
        }
    }

    pub fn toggle(&mut self) -> bool {
        self.visible = !self.visible;
        self.visible
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

    pub fn window_ids(&self) -> Vec<WindowId> {
        self.windows.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.windows.is_empty()
    }
}