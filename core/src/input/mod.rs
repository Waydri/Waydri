pub mod android_input;
pub mod gesture;
pub mod keyboard;
pub mod libinput;
pub mod pointer;
pub mod touch;

use crate::utils::Vec2;

use gesture::GestureRecognizer;
pub use android_input::AndroidInputEvent;
pub use gesture::{GestureEvent, GestureKind, GestureParams};
pub use keyboard::{KeyEvent, Keyboard};
pub use pointer::{Button, Pointer, ScrollAxis};
pub use touch::{Touch, TouchPoint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionSource {
    Mouse,
    Touchpad,
    Touchscreen,
    Stylus,
    Pen,
}

impl MotionSource {
    pub fn is_relative(self) -> bool {
        matches!(self, MotionSource::Mouse | MotionSource::Touchpad)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    Key { code: u32, down: bool },
    Pointer { button: Option<u32>, x: f32, y: f32, source: MotionSource },
    Touch { id: i32, down: bool, x: f32, y: f32 },
}

pub struct InputManager {
    pub keyboard: Keyboard,
    pub pointer: Pointer,
    pub touch: Touch,
    pub gestures: GestureRecognizer,
    pub events: Vec<InputEvent>,
    pub gesture_events: Vec<GestureEvent>,
    cursor: Vec2,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            keyboard: Keyboard::new(),
            pointer: Pointer::new(),
            touch: Touch::new(),
            gestures: GestureRecognizer::new(),
            events: Vec::new(),
            gesture_events: Vec::new(),
            cursor: Vec2::ZERO,
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        self.events.push(event);
    }

    pub fn dispatch(&mut self) {
        let events = std::mem::take(&mut self.events);
        for event in events {
            match event {
                InputEvent::Key { code, down } => {
                    if down {
                        self.keyboard.press(code);
                    } else {
                        self.keyboard.release(code);
                    }
                }
                InputEvent::Pointer { button, x, y, source } => {
                    let position = Vec2::new(x, y);
                    if source.is_relative() {
                        self.pointer.move_to(self.cursor + position);
                    } else {
                        self.pointer.move_to(position);
                    }
                    self.cursor = self.pointer.position;
                    if let Some(code) = button {
                        if let Some(btn) = Button::from_code(code) {
                            self.pointer.press(btn);
                        }
                    }
                }
                InputEvent::Touch { id, down, x, y } => {
                    let time = self.touch_clock();
                    let position = Vec2::new(x, y);
                    if down {
                        if self.touch.active_count() == 0 {
                            self.gesture_events
                                .push(self.gestures.on_touch_down(position, time, 1));
                        }
                        self.touch.down(id, position, time);
                    } else {
                        if let Some(point) = self.touch.up(id, time) {
                            if self.touch.active_count() == 0 {
                                self.gesture_events
                                    .push(self.gestures.on_touch_up(point.position, time));
                            }
                        }
                    }
                }
            }
        }
    }

    fn touch_clock(&self) -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs_f64())
            .unwrap_or(0.0)
    }

    pub fn take_gestures(&mut self) -> Vec<GestureEvent> {
        std::mem::take(&mut self.gesture_events)
    }

    pub fn position(&self) -> Vec2 {
        self.pointer.position
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.keyboard.clear();
        self.touch.cancel();
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}