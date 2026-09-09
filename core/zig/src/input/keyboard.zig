const std = @import("std");

pub const KeyEvent = struct {
    keycode: u32,
    state: KeyState,
    repeat: bool,
    time: u64,
    device_id: u32,

    pub const KeyState = enum(u32) {
        released = 0,
        pressed = 1,
        repeated = 2,
    };

    pub fn isPressed(self: KeyEvent) bool {
        return self.state == .pressed or self.state == .repeated;
    }

    pub fn isReleased(self: KeyEvent) bool {
        return self.state == .released;
    }

    pub fn isRepeat(self: KeyEvent) bool {
        return self.repeat;
    }

    pub fn key_to_string(keycode: u32) ?[:0]const u8 {
        return switch (keycode) {
            KEY_A => "a",
            KEY_B => "b",
            KEY_C => "c",
            KEY_D => "d",
            KEY_E => "e",
            KEY_F => "f",
            KEY_G => "g",
            KEY_H => "h",
            KEY_I => "i",
            KEY_J => "j",
            KEY_K => "k",
            KEY_L => "l",
            KEY_M => "m",
            KEY_N => "n",
            KEY_O => "o",
            KEY_P => "p",
            KEY_Q => "q",
            KEY_R => "r",
            KEY_S => "s",
            KEY_T => "t",
            KEY_U => "u",
            KEY_V => "v",
            KEY_W => "w",
            KEY_X => "x",
            KEY_Y => "y",
            KEY_Z => "z",
            KEY_0 => "0",
            KEY_1 => "1",
            KEY_2 => "2",
            KEY_3 => "3",
            KEY_4 => "4",
            KEY_5 => "5",
            KEY_6 => "6",
            KEY_7 => "7",
            KEY_8 => "8",
            KEY_9 => "9",
            KEY_SPACE => " ",
            KEY_ENTER => "\n",
            KEY_ESC => "escape",
            KEY_TAB => "\t",
            KEY_BACKSPACE => "backspace",
            KEY_DELETE => "delete",
            KEY_UP => "up",
            KEY_DOWN => "down",
            KEY_LEFT => "left",
            KEY_RIGHT => "right",
            KEY_HOME => "home",
            KEY_END => "end",
            KEY_PAGEUP => "pageup",
            KEY_PAGEDOWN => "pagedown",
            KEY_F1 => "F1",
            KEY_F2 => "F2",
            KEY_F3 => "F3",
            KEY_F4 => "F4",
            KEY_F5 => "F5",
            KEY_F6 => "F6",
            KEY_F7 => "F7",
            KEY_F8 => "F8",
            KEY_F9 => "F9",
            KEY_F10 => "F10",
            KEY_F11 => "F11",
            KEY_F12 => "F12",
            else => null,
        };
    }
};

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
pub const KEY_ENTER: u32 = 28;
pub const KEY_ESC: u32 = 1;
pub const KEY_SPACE: u32 = 57;
pub const KEY_TAB: u32 = 15;
pub const KEY_BACKSPACE: u32 = 14;
pub const KEY_DELETE: u32 = 111;
pub const KEY_UP: u32 = 103;
pub const KEY_DOWN: u32 = 108;
pub const KEY_LEFT: u32 = 105;
pub const KEY_RIGHT: u32 = 106;
pub const KEY_HOME: u32 = 102;
pub const KEY_END: u32 = 107;
pub const KEY_PAGEUP: u32 = 104;
pub const KEY_PAGEDOWN: u32 = 109;
pub const KEY_INSERT: u32 = 110;
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
pub const KEY_LEFT_SHIFT: u32 = 42;
pub const KEY_RIGHT_SHIFT: u32 = 54;
pub const KEY_LEFT_CTRL: u32 = 29;
pub const KEY_RIGHT_CTRL: u32 = 97;
pub const KEY_LEFT_ALT: u32 = 56;
pub const KEY_RIGHT_ALT: u32 = 100;
pub const KEY_LEFT_SUPER: u32 = 125;
pub const KEY_RIGHT_SUPER: u32 = 126;
pub const KEY_CAPS_LOCK: u32 = 58;
pub const KEY_NUM_LOCK: u32 = 69;
pub const KEY_SCROLL_LOCK: u32 = 70;
pub const KEY_COMPOSE: u32 = 127;
pub const KEY_PRINT: u32 = 99;
pub const KEY_PAUSE: u32 = 119;
pub const KEY_MENU: u32 = 139;
pub const KEY_VOLUMEUP: u32 = 115;
pub const KEY_VOLUMEDOWN: u32 = 114;
pub const KEY_MUTE: u32 = 113;
pub const KEY_POWER: u32 = 116;
pub const KEY_SLEEP: u32 = 142;
pub const KEY_WAKEUP: u32 = 143;
