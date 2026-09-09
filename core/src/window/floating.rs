use crate::utils::Rect;

use super::window::WindowId;

#[derive(Debug, Default)]
pub struct FloatingLayer {
    pub floating: Vec<WindowId>,
    pub saved: Vec<(WindowId, Option<Rect>)>,
}

impl FloatingLayer {
    pub fn new() -> Self {
        FloatingLayer {
            floating: Vec::new(),
            saved: Vec::new(),
        }
    }

    pub fn is_floating(&self, id: WindowId) -> bool {
        self.floating.contains(&id)
    }

    pub fn add(&mut self, id: WindowId, placement: Rect) {
        if !self.is_floating(id) {
            self.floating.push(id);
            self.saved.push((id, Some(placement)));
        }
    }

    pub fn remove(&mut self, id: WindowId) -> Option<Rect> {
        self.floating.retain(|w| *w != id);
        let mut saved_placement = None;
        self.saved.retain(|(w, rect)| {
            if *w == id {
                saved_placement = *rect;
                false
            } else {
                true
            }
        });
        saved_placement
    }

    pub fn toggle(&mut self, id: WindowId, placement: Rect) -> bool {
        if self.is_floating(id) {
            self.remove(id);
            false
        } else {
            self.add(id, placement);
            true
        }
    }

    pub fn clear(&mut self) {
        self.floating.clear();
        self.saved.clear();
    }
}