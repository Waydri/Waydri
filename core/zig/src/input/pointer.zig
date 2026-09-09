const std = @import("std");

pub const ButtonState = enum(u32) {
    released = 0,
    pressed = 1,

    pub fn isPressed(self: ButtonState) bool {
        return self == .pressed;
    }

    pub fn isReleased(self: ButtonState) bool {
        return self == .released;
    }
};

pub const AxisSource = enum(u32) {
    none = 0,
    finger = 1,
    continuous = 2,
    wheel = 3,
    wheel_tilt = 4,
};

pub const PointerEvent = struct {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    button: u32,
    state: ButtonState,
    axis_x: f64,
    axis_y: f64,
    axis_scroll: f64,
    time: u64,
    device_id: u32,
    seat_x: f64,
    seat_y: f64,
    surface_x: f64,
    surface_y: f64,
    axis_source: AxisSource,
    axis_discrete_x: i32,
    axis_discrete_y: i32,
    buttons_held: u32,
    modifiers: ModifierState,

    pub const ModifierState = packed struct(u32) {
        shift: bool = false,
        ctrl: bool = false,
        alt: bool = false,
        super: bool = false,
        caps_lock: bool = false,
        num_lock: bool = false,
        _padding: u26 = 0,

        pub fn any(self: ModifierState) bool {
            return self.shift or self.ctrl or self.alt or self.super;
        }

        pub fn none(self: ModifierState) bool {
            return !self.any();
        }
    };

    pub const BUTTON_LEFT: u32 = 0x110;
    pub const BUTTON_RIGHT: u32 = 0x111;
    pub const BUTTON_MIDDLE: u32 = 0x112;
    pub const BUTTON_SIDE: u32 = 0x113;
    pub const BUTTON_EXTRA: u32 = 0x114;
    pub const BUTTON_FORWARD: u32 = 0x115;
    pub const BUTTON_BACK: u32 = 0x116;

    pub fn isButtonPressed(self: PointerEvent, button: u32) bool {
        return self.button == button and self.state == .pressed;
    }

    pub fn isButtonReleased(self: PointerEvent, button: u32) bool {
        return self.button == button and self.state == .released;
    }

    pub fn isLeftButton(self: PointerEvent) bool {
        return self.button == BUTTON_LEFT;
    }

    pub fn isRightButton(self: PointerEvent) bool {
        return self.button == BUTTON_RIGHT;
    }

    pub fn isMiddleButton(self: PointerEvent) bool {
        return self.button == BUTTON_MIDDLE;
    }

    pub fn hasModifier(self: PointerEvent, mod: ModifierState) bool {
        return (self.modifiers & mod) != 0;
    }

    pub fn hasShift(self: PointerEvent) bool {
        return self.modifiers.shift;
    }

    pub fn hasCtrl(self: PointerEvent) bool {
        return self.modifiers.ctrl;
    }

    pub fn hasAlt(self: PointerEvent) bool {
        return self.modifiers.alt;
    }

    pub fn hasSuper(self: PointerEvent) bool {
        return self.modifiers.super;
    }

    pub fn movementDelta(self: PointerEvent) struct { dx: f64, dy: f64 } {
        return .{ .dx = self.dx, .dy = self.dy };
    }

    pub fn position(self: PointerEvent) struct { x: f64, y: f64 } {
        return .{ .x = self.x, .y = self.y };
    }

    pub fn surfacePosition(self: PointerEvent) struct { x: f64, y: f64 } {
        return .{ .x = self.surface_x, .y = self.surface_y };
    }

    pub fn axisDelta(self: PointerEvent) struct { x: f64, y: f64 } {
        return .{ .x = self.axis_x, .y = self.axis_y };
    }

    pub fn hasAxis(self: PointerEvent, source: AxisSource) bool {
        return self.axis_source == source;
    }

    pub fn isScroll(self: PointerEvent) bool {
        return self.axis_source == .wheel or self.axis_source == .wheel_tilt;
    }

    pub fn isTouchpad(self: PointerEvent) bool {
        return self.axis_source == .finger;
    }
};
