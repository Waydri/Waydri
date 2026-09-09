use super::InputEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceCapability {
    Keyboard,
    Pointer,
    Touchscreen,
    Touchpad,
    Button,
    Scroll,
    Switch,
    Gesture,
}

impl DeviceCapability {
    pub fn from_bits(bits: u32) -> Vec<DeviceCapability> {
        let mut caps = Vec::new();
        if bits & 0x01 != 0 {
            caps.push(DeviceCapability::Keyboard);
        }
        if bits & 0x02 != 0 {
            caps.push(DeviceCapability::Pointer);
        }
        if bits & 0x04 != 0 {
            caps.push(DeviceCapability::Touchscreen);
        }
        if bits & 0x08 != 0 {
            caps.push(DeviceCapability::Touchpad);
        }
        if bits & 0x10 != 0 {
            caps.push(DeviceCapability::Button);
        }
        if bits & 0x20 != 0 {
            caps.push(DeviceCapability::Scroll);
        }
        if bits & 0x40 != 0 {
            caps.push(DeviceCapability::Gesture);
        }
        caps
    }
}

#[derive(Debug, Clone)]
pub struct LibinputDevice {
    pub name: String,
    pub caps: Vec<DeviceCapability>,
    pub enabled: bool,
}

impl LibinputDevice {
    pub fn new(name: String, caps: Vec<DeviceCapability>) -> Self {
        LibinputDevice {
            name,
            caps,
            enabled: true,
        }
    }

    pub fn has(&self, cap: DeviceCapability) -> bool {
        self.caps.contains(&cap)
    }

    pub fn desktop_capabilities(name: &str) -> Vec<DeviceCapability> {
        let lower = name.to_ascii_lowercase();
        let mut caps = Vec::new();
        if lower.contains("touchscreen") {
            caps.push(DeviceCapability::Touchscreen);
        }
        if lower.contains("touchpad") || lower.contains("trackpad") {
            caps.push(DeviceCapability::Touchpad);
            caps.push(DeviceCapability::Gesture);
        }
        if lower.contains("keyboard") {
            caps.push(DeviceCapability::Keyboard);
        }
        if lower.contains("mouse") || lower.contains("pointer") {
            caps.push(DeviceCapability::Pointer);
            caps.push(DeviceCapability::Button);
            caps.push(DeviceCapability::Scroll);
        }
        if lower.contains("button") || lower.contains("wheel") {
            caps.push(DeviceCapability::Button);
            caps.push(DeviceCapability::Scroll);
        }
        if caps.is_empty() {
            caps.push(DeviceCapability::Switch);
        }
        caps
    }

    pub fn translate(&self, _event: u64) -> Option<InputEvent> {
        if !self.enabled {
            return None;
        }
        if self.has(DeviceCapability::Touchscreen) {
            Some(InputEvent::Touch { id: 0, down: true, x: 0.0, y: 0.0 })
        } else {
            None
        }
    }
}