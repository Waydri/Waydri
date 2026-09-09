use crate::utils::{Rect, Vec2};
use std::collections::VecDeque;

use super::state::{CompositorEvent, CompositorState, SurfaceId};

pub struct CompositorProcessor {
    events: VecDeque<CompositorEvent>,
}

impl CompositorProcessor {
    pub fn new() -> Self {
        CompositorProcessor {
            events: VecDeque::new(),
        }
    }

    pub fn push(&mut self, event: CompositorEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<CompositorEvent> {
        self.events.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}

impl Default for CompositorProcessor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CompositorHandle {
    pub id: SurfaceId,
    pub signaled: bool,
}

impl CompositorHandle {
    pub fn new(id: SurfaceId) -> Self {
        CompositorHandle {
            id,
            signaled: false,
        }
    }
}

pub struct CompositorHandler;

impl CompositorHandler {
    pub fn add_surface(state: &mut CompositorState) -> SurfaceId {
        state.create_surface()
    }

    pub fn remove_surface(state: &mut CompositorState, id: SurfaceId) {
        state.destroy_surface(id);
    }

    pub fn on_frame(state: &mut CompositorState, frame: Rect) {
        for id in state.mapped_surfaces() {
            if let Some(surface) = state.surfaces.get_mut(&id) {
                surface.damage = frame;
            }
        }
        state.dirty = true;
    }

    pub fn update_position(state: &mut CompositorState, surface: SurfaceId, position: Vec2) {
        if let Some(s) = state.surfaces.get_mut(&surface) {
            s.damage = Rect::new(position.x as i32, position.y as i32, s.width, s.height);
        }
    }

    pub fn dispatch(state: &mut CompositorState, event: CompositorEvent) {
        match event {
            CompositorEvent::SurfaceCreated(id) => state.set_role(id, super::state::SurfaceRole::Shell),
            CompositorEvent::SurfaceDamaged(_) => state.dirty = true,
            CompositorEvent::SurfaceCommitted(_) => state.dirty = true,
            CompositorEvent::SurfaceDestroyed(id) => {
                state.destroy_surface(id);
            }
            CompositorEvent::Frame(_) => state.dirty = true,
        }
    }
}