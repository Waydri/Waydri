use crate::utils::Rect;

#[derive(Debug, Clone, Default)]
pub struct DamageTracker {
    regions: Vec<Rect>,
    full: bool,
}

impl DamageTracker {
    pub fn new() -> Self {
        DamageTracker {
            regions: Vec::new(),
            full: false,
        }
    }

    pub fn add_rect(&mut self, rect: Rect) {
        if rect.is_empty() {
            return;
        }
        if self.full {
            return;
        }
        if self.regions.len() >= 64 {
            self.full = true;
            self.regions.clear();
            return;
        }
        self.regions.push(rect);
    }

    pub fn mark_full(&mut self) {
        self.full = true;
        self.regions.clear();
    }

    pub fn take_rects(&mut self) -> Vec<Rect> {
        if self.full {
            self.full = false;
            return Vec::new();
        }
        std::mem::take(&mut self.regions)
    }

    pub fn is_empty(&self) -> bool {
        !self.full && self.regions.is_empty()
    }

    pub fn union(&mut self, bounds: Rect) -> Rect {
        let mut combined = Rect::new(bounds.x, bounds.y, 0, 0);
        for region in &self.regions {
            combined = combined.union(&region);
        }
        combined
    }

    pub fn len(&self) -> usize {
        self.regions.len()
    }
}