use std::collections::HashMap;

use crate::compositor::CompositorState;
use crate::renderer::SoftwareRenderer;
use crate::utils::{Color, Rect, Transform};
use crate::window::WindowManager;

use super::{BackendKind, Output, OutputMode};

pub struct OutputManager {
    pub outputs: HashMap<u64, Output>,
    next_id: u64,
    focused: Option<u64>,
}

impl Default for OutputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputManager {
    pub fn new() -> Self {
        OutputManager {
            outputs: HashMap::new(),
            next_id: 1,
            focused: None,
        }
    }

    pub fn add_headless(&mut self, name: String, width: u32, height: u32, refresh: u32) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let output = Output::headless(id, name, width, height, refresh);
        if self.focused.is_none() {
            self.focused = Some(id);
        }
        self.outputs.insert(id, output);
        id
    }

    pub fn add_output(&mut self, output: Output) -> u64 {
        let id = output.id;
        self.outputs.insert(id, output);
        if self.focused.is_none() {
            self.focused = Some(id);
        }
        id
    }

    pub fn remove_output(&mut self, id: u64) -> bool {
        self.outputs.remove(&id).is_some()
    }

    pub fn output(&self, id: u64) -> Option<&Output> {
        self.outputs.get(&id)
    }

    pub fn output_mut(&mut self, id: u64) -> Option<&mut Output> {
        self.outputs.get_mut(&id)
    }

    pub fn focused(&self) -> Option<&Output> {
        self.focused.and_then(|id| self.outputs.get(&id))
    }

    pub fn set_focused(&mut self, id: u64) {
        if self.outputs.contains_key(&id) {
            self.focused = Some(id);
        }
    }

    pub fn count(&self) -> usize {
        self.outputs.len()
    }

    pub fn total_area(&self) -> Rect {
        let mut combined = Rect::default();
        for output in self.outputs.values() {
            if output.enabled {
                combined = combined.union(&output.geometry());
            }
        }
        combined
    }

    pub fn focused_geometry(&self) -> Rect {
        self.focused()
            .map(|o| o.geometry())
            .or_else(|| self.outputs.values().next().map(|o| o.geometry()))
            .unwrap_or_default()
    }

    pub fn set_transform(&mut self, id: u64, transform: Transform) {
        if let Some(output) = self.outputs.get_mut(&id) {
            output.transform = transform;
        }
    }

    pub fn set_mode(&mut self, id: u64, mode: OutputMode) {
        if let Some(output) = self.outputs.get_mut(&id) {
            output.set_mode(mode);
        }
    }

    pub fn clone_geometries(&self) -> Vec<Rect> {
        self.outputs.values().map(|o| o.geometry()).collect()
    }

    pub fn compose(
        &self,
        renderer: &mut SoftwareRenderer,
        _compositor: &CompositorState,
        window_manager: &WindowManager,
    ) {
        renderer.clear();
        let bounds = self.focused_geometry();
        let accent = Color::from_hex(0xCBA6F7);
        let focused_id = window_manager.focused();
        for window_id in window_manager.visible_sorted() {
            let Some(window) = window_manager.window(window_id) else {
                continue;
            };
            let rect = window.placement;
            if !rect.intersects(&bounds) {
                continue;
            }
            renderer.fill_rect(rect, window.color);
            if let Some(buffer) = &window.buffer {
                let (width, height) = window.buffer_size;
                renderer.blit_surface(buffer, width, height, rect, window.alpha);
            }
            renderer.apply_rounded(rect, 8);
            if focused_id == Some(window_id) {
                draw_border(renderer, rect, 2, accent);
            }
        }
    }
}

fn draw_border(renderer: &mut SoftwareRenderer, rect: Rect, thickness: i32, color: Color) {
    if thickness <= 0 || rect.width <= 0 || rect.height <= 0 {
        return;
    }
    renderer.fill_rect(Rect::new(rect.x, rect.y, rect.width, thickness), color);
    renderer.fill_rect(
        Rect::new(rect.x, rect.y + rect.height - thickness, rect.width, thickness),
        color,
    );
    renderer.fill_rect(Rect::new(rect.x, rect.y, thickness, rect.height), color);
    renderer.fill_rect(
        Rect::new(rect.x + rect.width - thickness, rect.y, thickness, rect.height),
        color,
    );
}

pub struct BackendContext {
    pub name: BackendKind,
    pub default_modes: Vec<OutputMode>,
}

impl BackendContext {
    pub fn new(kind: BackendKind) -> Self {
        BackendContext {
            name: kind,
            default_modes: vec![
                OutputMode::new(1920, 1080, 120),
                OutputMode::new(1366, 768, 60),
                OutputMode::new(1280, 720, 60),
                OutputMode::new(800, 600, 60),
            ],
        }
    }

    pub fn suitable_modes(&self, width: u32, height: u32) -> Vec<OutputMode> {
        self.default_modes
            .iter()
            .copied()
            .filter(|m| m.width <= width && m.height <= height)
            .collect()
    }
}