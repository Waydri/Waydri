use crate::utils::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeatCapability {
    Pointer,
    Keyboard,
    Touch,
}

impl SeatCapability {
    pub fn masks(capabilities: &[SeatCapability]) -> u32 {
        let mut mask = 0;
        for capability in capabilities {
            match capability {
                SeatCapability::Pointer => mask |= 1,
                SeatCapability::Keyboard => mask |= 2,
                SeatCapability::Touch => mask |= 4,
            }
        }
        mask
    }
}

#[derive(Debug, Clone)]
pub struct Seat {
    pub name: String,
    pub capabilities: Vec<SeatCapability>,
    pub focused_client: u32,
    pub pointer_position: Vec2,
    pub pressed_keys: Vec<u32>,
    pub pointer_serial: u32,
    pub keyboard_serial: u32,
}

impl Seat {
    pub fn new(name: String, capabilities: Vec<SeatCapability>) -> Self {
        Seat {
            name,
            capabilities,
            focused_client: 0,
            pointer_position: Vec2::ZERO,
            pressed_keys: Vec::new(),
            pointer_serial: 0,
            keyboard_serial: 0,
        }
    }

    pub fn has(&self, capability: SeatCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn focus(&mut self, client: u32) {
        self.focused_client = client;
    }

    pub fn next_pointer_serial(&mut self) -> u32 {
        self.pointer_serial = self.pointer_serial.wrapping_add(1);
        self.pointer_serial
    }

    pub fn next_keyboard_serial(&mut self) -> u32 {
        self.keyboard_serial = self.keyboard_serial.wrapping_add(1);
        self.keyboard_serial
    }
}

pub fn has_full_capabilities(mask: u32) -> bool {
    mask & 0x7 == 0x7
}