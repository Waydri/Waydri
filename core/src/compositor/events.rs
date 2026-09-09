use super::state::SurfaceId;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct FrameRequest {
    pub surface: SurfaceId,
    pub sync: bool,
}

#[derive(Debug)]
pub struct EventQueue {
    pub pending: VecDeque<FrameRequest>,
    pub frame_due: bool,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            pending: VecDeque::new(),
            frame_due: false,
        }
    }

    pub fn request_frame(&mut self, request: FrameRequest) {
        self.pending.push_back(request);
    }

    pub fn pop(&mut self) -> Option<FrameRequest> {
        self.pending.pop_front()
    }

    pub fn drain(&mut self) -> Vec<FrameRequest> {
        self.pending.drain(..).collect()
    }

    pub fn schedule_frame(&mut self) {
        self.frame_due = true;
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CompositorProcessor: Send {
    fn process_frame(&mut self, frame: &FrameRequest);
}