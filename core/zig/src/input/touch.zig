const std = @import("std");

pub const TouchState = enum {
    down,
    move,
    up,

    pub fn isDown(self: TouchState) bool {
        return self == .down;
    }

    pub fn isMove(self: TouchState) bool {
        return self == .move;
    }

    pub fn isUp(self: TouchState) bool {
        return self == .up;
    }
};

pub const TouchEvent = struct {
    slot: i32,
    x: f64,
    y: f64,
    pressure: f64,
    touch_id: u32,
    state: TouchState,
    time: u64,
    device_id: u32,
    major: f64,
    minor: f64,
    orientation: f64,
    tool_type: ToolType,

    pub const ToolType = enum {
        finger,
        pen,
        eraser,
        mouse,
        lens,
        outline,

        pub fn name(self: ToolType) []const u8 {
            return switch (self) {
                .finger => "Finger",
                .pen => "Pen",
                .eraser => "Eraser",
                .mouse => "Mouse",
                .lens => "Lens",
                .outline => "Outline",
            };
        }

        pub fn isStylus(self: ToolType) bool {
            return self == .pen or self == .eraser;
        }
    };

    pub fn isFinger(self: TouchEvent) bool {
        return self.tool_type == .finger;
    }

    pub fn isStylus(self: TouchEvent) bool {
        return self.tool_type.isStylus();
    }

    pub fn hasPressure(self: TouchEvent) bool {
        return self.pressure > 0.0;
    }

    pub fn position(self: TouchEvent) struct { x: f64, y: f64 } {
        return .{ .x = self.x, .y = self.y };
    }

    pub fn size(self: TouchEvent) struct { major: f64, minor: f64 } {
        return .{ .major = self.major, .minor = self.minor };
    }

    pub fn normalizedPressure(self: TouchEvent) f64 {
        return @max(0.0, @min(1.0, self.pressure));
    }

    pub fn isActive(self: TouchEvent) bool {
        return self.state == .down or self.state == .move;
    }
};
