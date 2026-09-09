use crate::utils::Rect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum Event {
    WindowCreated { id: u64, title: String },
    WindowDestroyed { id: u64 },
    WindowFocusChanged { id: u64 },
    WorkspaceChanged { index: usize },
    LayoutChanged { name: String },
    OutputAdded { name: String },
    OutputRemoved { name: String },
    FrameCompleted { fps: f32 },
    DamagedRegion { x: i32, y: i32, width: i32, height: i32 },
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::WindowCreated { .. } => "window_created",
            Event::WindowDestroyed { .. } => "window_destroyed",
            Event::WindowFocusChanged { .. } => "window_focus_changed",
            Event::WorkspaceChanged { .. } => "workspace_changed",
            Event::LayoutChanged { .. } => "layout_changed",
            Event::OutputAdded { .. } => "output_added",
            Event::OutputRemoved { .. } => "output_removed",
            Event::FrameCompleted { .. } => "frame_completed",
            Event::DamagedRegion { .. } => "damaged_region",
        }
    }
}

pub struct EventSink {
    pub events: Vec<Event>,
    pub capacity: usize,
}

impl EventSink {
    pub fn new(capacity: usize) -> Self {
        EventSink {
            events: Vec::new(),
            capacity,
        }
    }

    pub fn push(&mut self, event: Event) {
        if self.events.len() >= self.capacity {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

pub fn damage_to_event(rect: Rect, output: &str) -> Event {
    let _ = output;
    Event::DamagedRegion {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}