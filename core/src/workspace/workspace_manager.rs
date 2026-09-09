use crate::window::WindowId;

use super::workspace::Workspace;

#[derive(Debug)]
pub struct WorkspaceManager {
    pub workspaces: Vec<Workspace>,
    pub active: usize,
}

impl WorkspaceManager {
    pub fn new(count: usize) -> Self {
        let workspaces = (1..=count.max(1))
            .map(|index| Workspace::new(index as u64, format!("workspace {index}")))
            .collect();
        WorkspaceManager {
            workspaces,
            active: 0,
        }
    }

    pub fn active_workspace(&self) -> &Workspace {
        &self.workspaces[self.active]
    }

    pub fn active_workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active]
    }

    pub fn workspace(&self, index: usize) -> Option<&Workspace> {
        self.workspaces.get(index)
    }

    pub fn workspace_mut(&mut self, index: usize) -> Option<&mut Workspace> {
        self.workspaces.get_mut(index)
    }

    pub fn switch(&mut self, index: usize) -> bool {
        if index < self.workspaces.len() {
            self.active = index;
            true
        } else {
            false
        }
    }

    pub fn next(&mut self) -> usize {
        self.active = (self.active + 1) % self.workspaces.len();
        self.active
    }

    pub fn previous(&mut self) -> usize {
        self.active = if self.active == 0 {
            self.workspaces.len() - 1
        } else {
            self.active - 1
        };
        self.active
    }

    pub fn add_window(&mut self, window: WindowId) {
        self.active_workspace_mut().add_window(window);
    }

    pub fn move_window(&mut self, window: WindowId, target: usize) -> bool {
        if target >= self.workspaces.len() {
            return false;
        }
        for ws in self.workspaces.iter_mut() {
            ws.remove_window(window);
        }
        self.workspaces[target].add_window(window);
        true
    }

    pub fn set_layout(&mut self, layout: &str) {
        self.active_workspace_mut().set_layout(layout);
    }

    pub fn layout_name(&self) -> &str {
        self.active_workspace().layout_name()
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn count(&self) -> usize {
        self.workspaces.len()
    }

    pub fn toggle_workspace_visibility(&mut self, index: usize) -> bool {
        if let Some(ws) = self.workspaces.get_mut(index) {
            ws.visible = !ws.visible;
            return ws.visible;
        }
        false
    }
}