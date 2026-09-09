use super::{InputEvent, MotionSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidInputClass {
    Touchscreen,
    Mouse,
    Trackpad,
    Joystick,
    Stylus,
    Keyboard,
}

impl AndroidInputClass {
    pub fn from_source(source: i32) -> AndroidInputClass {
        if source & 0x0002 != 0 {
            AndroidInputClass::Touchscreen
        } else if source & 0x2002 != 0 {
            AndroidInputClass::Mouse
        } else if source & 0x00100000 != 0 {
            AndroidInputClass::Joystick
        } else if source & 0x00000002 != 0 {
            AndroidInputClass::Trackpad
        } else if source & 0x00004000 != 0 {
            AndroidInputClass::Stylus
        } else {
            AndroidInputClass::Keyboard
        }
    }
}

pub const SOURCE_TOUCHSCREEN: i32 = 0x00001002;
pub const SOURCE_MOUSE: i32 = 0x00002002;
pub const SOURCE_TRACKBALL: i32 = 0x00040000;
pub const SOURCE_STYLUS: i32 = 0x00004002;
pub const SOURCE_JOYSTICK: i32 = 0x01000010;
pub const SOURCE_KEYBOARD: i32 = 0x00000301;

pub struct AndroidInputEvent {
    pub source: i32,
    pub action: i32,
    pub pointer_id: i32,
    pub x: f32,
    pub y: f32,
    pub button: i32,
}

impl AndroidInputEvent {
    pub fn as_input_event(&self) -> InputEvent {
        let class = AndroidInputClass::from_source(self.source);
        match class {
            AndroidInputClass::Touchscreen => InputEvent::Touch {
                id: self.pointer_id,
                down: self.action == 0,
                x: self.x,
                y: self.y,
            },
            AndroidInputClass::Mouse | AndroidInputClass::Trackpad => InputEvent::Pointer {
                button: if self.button != 0 {
                    Some(self.button as u32)
                } else {
                    None
                },
                x: self.x,
                y: self.y,
                source: MotionSource::Mouse,
            },
            _ => InputEvent::Key {
                code: self.pointer_id as u32,
                down: self.action != 1,
            },
        }
    }
}

pub fn describe_device(class: AndroidInputClass, vendor: &str, name: &str) -> String {
    format!("{vendor:?} {name} ({class:?})")
}