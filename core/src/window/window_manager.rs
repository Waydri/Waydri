use std::collections::HashMap;

use super::window::{Window, WindowId};

#[derive(Debug)]
pub struct WindowManager {
    pub windows: HashMap<WindowId, Window>,
    focus_order: Vec<WindowId>,
    next_id: u64,
    next_depth: u64,
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        WindowManager {
            windows: HashMap::new(),
            focus_order: Vec::new(),
            next_id: 1,
            next_depth: 0,
        }
    }

    pub fn create_window(&mut self, title: String, app_id: String, workspace: u64) -> WindowId {
        let id = WindowId(self.next_id);
        self.next_id += 1;
        self.next_depth += 1;
        let mut window = Window::new(id, title, app_id, workspace);
        window.depth = self.next_depth;
        self.windows.insert(id, window);
        self.focus(id);
        id
    }

    pub fn destroy_window(&mut self, id: WindowId) -> bool {
        self.focus_order.retain(|w| *w != id);
        self.windows.remove(&id).is_some()
    }

    pub fn window(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    pub fn focus(&mut self, id: WindowId) {
        if !self.windows.contains_key(&id) {
            return;
        }
        self.focus_order.retain(|w| *w != id);
        self.focus_order.push(id);
    }

    pub fn focused(&self) -> Option<WindowId> {
        self.focus_order.last().copied()
    }

    pub fn set_placement(&mut self, id: WindowId, rect: crate::utils::Rect) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.placement = rect;
        }
    }

    pub fn set_alpha(&mut self, id: WindowId, alpha: f32) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.alpha = alpha.clamp(0.0, 1.0);
        }
    }

    pub fn set_title(&mut self, id: WindowId, title: String) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.title = title;
        }
    }

    pub fn count(&self) -> usize {
        self.windows.len()
    }

    pub fn visible_sorted(&self) -> Vec<WindowId> {
        let mut visible: Vec<WindowId> = self
            .windows
            .iter()
            .filter(|(_, w)| w.visible)
            .map(|(id, _)| *id)
            .collect();
        visible.sort_by_key(|id| self.windows[id].depth);
        visible
    }

    pub fn windows_on(&self, workspace: u64) -> Vec<WindowId> {
        let mut ids: Vec<WindowId> = self
            .windows
            .iter()
            .filter(|(_, w)| w.workspace == workspace)
            .map(|(id, _)| *id)
            .collect();
        ids.sort_by_key(|id| self.windows[id].depth);
        ids
    }

    pub fn for_each_window<F: FnMut(&Window)>(&self, mut f: F) {
        for window in self.windows.values() {
            f(window);
        }
    }

    pub fn ids(&self) -> Vec<WindowId> {
        self.windows.keys().copied().collect()
    }
}