use std::collections::HashMap;

use crate::utils::{Rect, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceRole {
    Shell,
    Layer,
    WorkspaceBackground,
    WorkspacePopup,
    Cursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    XdgToplevel,
    XdgPopup,
    LayerSurface,
    Subsurface,
    OsSurface,
}

#[derive(Debug, Clone)]
pub struct Surface {
    pub id: SurfaceId,
    pub width: i32,
    pub height: i32,
    pub buffer: Option<Vec<u8>>,
    pub mapped: bool,
    pub alpha: f32,
    pub transform: crate::utils::Transform,
    pub damage: Rect,
    pub role: SurfaceRole,
    pub kind: SurfaceKind,
    pub serial: u64,
}

impl Surface {
    pub fn new(id: SurfaceId) -> Self {
        Surface {
            id,
            width: 0,
            height: 0,
            buffer: None,
            mapped: false,
            alpha: 1.0,
            transform: crate::utils::Transform::Normal,
            damage: Rect::default(),
            role: SurfaceRole::Shell,
            kind: SurfaceKind::XdgToplevel,
            serial: 0,
        }
    }

    pub fn attach(&mut self, buffer: Vec<u8>, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.buffer = Some(buffer);
        self.damage = Rect::new(0, 0, width, height);
        self.mapped = true;
        self.serial += 1;
    }

    pub fn damage_rect(&self) -> Rect {
        self.damage
    }

    pub fn is_ready(&self) -> bool {
        self.mapped && self.buffer.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorEvent {
    SurfaceCreated(SurfaceId),
    SurfaceDamaged(SurfaceId),
    SurfaceCommitted(SurfaceId),
    SurfaceDestroyed(SurfaceId),
    Frame(SurfaceId),
}

#[derive(Debug)]
pub struct CompositorState {
    pub surfaces: HashMap<SurfaceId, Surface>,
    next_id: u64,
    pub dirty: bool,
}

impl Default for CompositorState {
    fn default() -> Self {
        Self::new()
    }
}

impl CompositorState {
    pub fn new() -> Self {
        CompositorState {
            surfaces: HashMap::new(),
            next_id: 1,
            dirty: false,
        }
    }

    pub fn create_surface(&mut self) -> SurfaceId {
        let id = SurfaceId(self.next_id);
        self.next_id += 1;
        self.surfaces.insert(id, Surface::new(id));
        self.dirty = true;
        id
    }

    pub fn attach_buffer(&mut self, id: SurfaceId, buffer: Vec<u8>, width: i32, height: i32) {
        if let Some(surface) = self.surfaces.get_mut(&id) {
            surface.attach(buffer, width, height);
            self.dirty = true;
        }
    }

    pub fn destroy_surface(&mut self, id: SurfaceId) -> bool {
        if self.surfaces.remove(&id).is_some() {
            self.dirty = true;
            true
        } else {
            false
        }
    }

    pub fn surface(&self, id: SurfaceId) -> Option<&Surface> {
        self.surfaces.get(&id)
    }

    pub fn surface_mut(&mut self, id: SurfaceId) -> Option<&mut Surface> {
        self.surfaces.get_mut(&id)
    }

    pub fn surface_count(&self) -> usize {
        self.surfaces.len()
    }

    pub fn mapped_count(&self) -> usize {
        self.surfaces.values().filter(|s| s.mapped).count()
    }

    pub fn mapped_surfaces(&self) -> Vec<SurfaceId> {
        self.surfaces
            .iter()
            .filter(|(_, s)| s.mapped)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn roles(&self) -> Vec<(SurfaceId, SurfaceRole)> {
        self.surfaces
            .iter()
            .map(|(id, s)| (*id, s.role))
            .collect()
    }

    pub fn set_role(&mut self, id: SurfaceId, role: SurfaceRole) {
        if let Some(s) = self.surfaces.get_mut(&id) {
            s.role = role;
        }
    }

    pub fn set_alpha(&mut self, id: SurfaceId, alpha: f32) {
        if let Some(s) = self.surfaces.get_mut(&id) {
            s.alpha = alpha.clamp(0.0, 1.0);
        }
    }

    pub fn surface_at(&self, position: Vec2) -> Option<SurfaceId> {
        self.surfaces
            .iter()
            .filter(|(_, s)| s.mapped)
            .max_by_key(|(id, s)| {
                let center_y = s.damage.center().y;
                let center_x = s.damage.center().x;
                let _ = id;
                ((s.damage.contains_point(position)) as u32, center_y as i64, center_x as i64)
            })
            .filter(|(_, s)| s.damage.contains_point(position))
            .map(|(id, _)| *id)
    }

    pub fn clear(&mut self) {
        self.surfaces.clear();
        self.next_id = 1;
        self.dirty = false;
    }
}