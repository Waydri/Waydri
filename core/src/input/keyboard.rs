pub const KEY_UNKNOWN: u32 = 0;
pub const KEY_ESC: u32 = 1;
pub const KEY_ENTER: u32 = 28;
pub const KEY_SPACE: u32 = 57;
pub const KEY_BACKSPACE: u32 = 14;
pub const KEY_TAB: u32 = 15;
pub const KEY_LEFT_SHIFT: u32 = 42;
pub const KEY_RIGHT_SHIFT: u32 = 54;
pub const KEY_LEFT_CTRL: u32 = 29;
pub const KEY_LEFT_ALT: u32 = 56;
pub const KEY_CAPS_LOCK: u32 = 58;
pub const KEY_A: u32 = 30;
pub const KEY_B: u32 = 48;
pub const KEY_C: u32 = 46;
pub const KEY_D: u32 = 32;
pub const KEY_E: u32 = 18;
pub const KEY_F: u32 = 33;
pub const KEY_G: u32 = 34;
pub const KEY_H: u32 = 35;
pub const KEY_I: u32 = 23;
pub const KEY_J: u32 = 36;
pub const KEY_K: u32 = 37;
pub const KEY_L: u32 = 38;
pub const KEY_M: u32 = 50;
pub const KEY_N: u32 = 49;
pub const KEY_O: u32 = 24;
pub const KEY_P: u32 = 25;
pub const KEY_Q: u32 = 16;
pub const KEY_R: u32 = 19;
pub const KEY_S: u32 = 31;
pub const KEY_T: u32 = 20;
pub const KEY_U: u32 = 22;
pub const KEY_V: u32 = 47;
pub const KEY_W: u32 = 17;
pub const KEY_X: u32 = 45;
pub const KEY_Y: u32 = 21;
pub const KEY_Z: u32 = 44;
pub const KEY_0: u32 = 11;
pub const KEY_1: u32 = 2;
pub const KEY_2: u32 = 3;
pub const KEY_3: u32 = 4;
pub const KEY_4: u32 = 5;
pub const KEY_5: u32 = 6;
pub const KEY_6: u32 = 7;
pub const KEY_7: u32 = 8;
pub const KEY_8: u32 = 9;
pub const KEY_9: u32 = 10;
pub const KEY_LEFT: u32 = 105;
pub const KEY_RIGHT: u32 = 106;
pub const KEY_UP: u32 = 103;
pub const KEY_DOWN: u32 = 108;
pub const KEY_HOME: u32 = 102;
pub const KEY_END: u32 = 107;
pub const KEY_PAGE_UP: u32 = 104;
pub const KEY_PAGE_DOWN: u32 = 109;
pub const KEY_F1: u32 = 59;
pub const KEY_F2: u32 = 60;
pub const KEY_F3: u32 = 61;
pub const KEY_F4: u32 = 62;
pub const KEY_F5: u32 = 63;
pub const KEY_F6: u32 = 64;
pub const KEY_F7: u32 = 65;
pub const KEY_F8: u32 = 66;
pub const KEY_F9: u32 = 67;
pub const KEY_F10: u32 = 68;
pub const KEY_F11: u32 = 87;
pub const KEY_F12: u32 = 88;

#[derive(Debug, Default)]
pub struct Keyboard {
    pub pressed: Vec<u32>,
    pub modifiers: Modifiers,
    pub repeat_count: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub caps_lock: bool,
    pub num_lock: bool,
}

impl Modifiers {
    pub fn is_empty(&self) -> bool {
        !self.shift && !self.ctrl && !self.alt && !self.meta && !self.caps_lock
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed: Vec::new(),
            modifiers: Modifiers::default(),
            repeat_count: 0,
        }
    }

    pub fn press(&mut self, key: u32) -> bool {
        if !self.pressed.contains(&key) {
            self.pressed.push(key);
            self.update_modifiers();
            true
        } else {
            self.repeat_count += 1;
            false
        }
    }

    pub fn release(&mut self, key: u32) -> bool {
        let before = self.pressed.len();
        self.pressed.retain(|k| *k != key);
        self.repeat_count = 0;
        self.update_modifiers();
        self.pressed.len() != before
    }

    pub fn is_pressed(&self, key: u32) -> bool {
        self.pressed.contains(&key)
    }

    pub fn clear(&mut self) {
        self.pressed.clear();
        self.repeat_count = 0;
        self.update_modifiers();
    }

    fn update_modifiers(&mut self) {
        self.modifiers.shift =
            self.is_pressed(KEY_LEFT_SHIFT) || self.is_pressed(KEY_RIGHT_SHIFT);
        self.modifiers.ctrl = self.is_pressed(KEY_LEFT_CTRL);
        self.modifiers.alt = self.is_pressed(KEY_LEFT_ALT);
        self.modifiers.caps_lock = self.is_pressed(KEY_CAPS_LOCK);
    }
}

pub fn key_name(key: u32) -> &'static str {
    match key {
        KEY_ESC => "escape",
        KEY_ENTER => "return",
        KEY_SPACE => "space",
        KEY_BACKSPACE => "backspace",
        KEY_TAB => "tab",
        KEY_LEFT_SHIFT => "left shift",
        KEY_RIGHT_SHIFT => "right shift",
        KEY_LEFT_CTRL => "left control",
        KEY_LEFT_ALT => "left alt",
        KEY_CAPS_LOCK => "caps lock",
        KEY_A..=KEY_Z => "letter",
        KEY_9..=KEY_0 => "digit",
        KEY_LEFT => "left",
        KEY_RIGHT => "right",
        KEY_UP => "up",
        KEY_DOWN => "down",
        KEY_HOME => "home",
        KEY_END => "end",
        KEY_F1..=KEY_F12 => "function",
        _ => "unknown",
    }
}

pub fn key_to_char(key: u32, shift: bool) -> Option<char> {
    let base = match key {
        KEY_A..=KEY_Z => (b'a' + (key - KEY_A) as u8) as char,
        KEY_9..=KEY_0 => (b'0' + (key - KEY_9) as u8) as char,
        KEY_SPACE => ' ',
        _ => return None,
    };
    if shift && base.is_alphabetic() {
        Some(base.to_ascii_uppercase())
    } else {
        Some(base)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Press,
    Release,
    Repeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: u32,
    pub action: KeyAction,
    pub modifiers: Modifiers,
}