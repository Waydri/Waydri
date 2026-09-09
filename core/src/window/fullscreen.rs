use crate::utils::Rect;

use super::window::WindowId;

#[derive(Debug, Default)]
pub struct FullscreenState {
    pub window: Option<WindowId>,
    pub stored_placement: Option<Rect>,
}

impl FullscreenState {
    pub fn new() -> Self {
        FullscreenState {
            window: None,
            stored_placement: None,
        }
    }

    pub fn enter(&mut self, id: WindowId, placement: Rect) {
        self.window = Some(id);
        self.stored_placement = Some(placement);
    }

    pub fn exit(&mut self) -> Option<(WindowId, Rect)> {
        let id = self.window.take()?;
        let placement = self.stored_placement.take().unwrap_or_default();
        Some((id, placement))
    }

    pub fn active(&self) -> bool {
        self.window.is_some()
    }

    pub fn toggle(&mut self, id: WindowId, placement: Rect) -> bool {
        if self.active() {
            self.exit();
            false
        } else {
            self.enter(id, placement);
            true
        }
    }
}