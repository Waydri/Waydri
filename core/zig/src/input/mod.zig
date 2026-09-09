pub const gesture = @import("gesture.zig");
pub const keyboard = @import("keyboard.zig");
pub const pointer = @import("pointer.zig");
pub const touch = @import("touch.zig");

pub const InputEvent = union(enum) {
    key: keyboard.KeyEvent,
    pointer: pointer.PointerEvent,
    touch: touch.TouchEvent,
    gesture: gesture.GestureState,

    pub fn timestamp(self: InputEvent) u64 {
        return switch (self) {
            .key => |e| e.time,
            .pointer => |e| e.time,
            .touch => |e| e.time,
            .gesture => 0,
        };
    }

    pub fn deviceId(self: InputEvent) u32 {
        return switch (self) {
            .key => |e| e.device_id,
            .pointer => |e| e.device_id,
            .touch => |e| e.device_id,
            .gesture => 0,
        };
    }

    pub fn isValid(self: InputEvent) bool {
        return switch (self) {
            .key => |e| e.keycode != 0,
            .pointer => true,
            .touch => |e| e.slot >= 0,
            .gesture => |s| s != .idle,
        };
    }
};
