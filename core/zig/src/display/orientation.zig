const std = @import("std");

pub const Orientation = enum(u8) {
    normal = 0,
    rot90 = 1,
    rot180 = 2,
    rot270 = 3,
    flipped = 4,
    flipped_rot90 = 5,
    flipped_rot180 = 6,
    flipped_rot270 = 7,

    pub fn degrees(self: Orientation) u32 {
        return switch (self) {
            .normal, .flipped => 0,
            .rot90, .flipped_rot90 => 90,
            .rot180, .flipped_rot180 => 180,
            .rot270, .flipped_rot270 => 270,
        };
    }

    pub fn isFlipped(self: Orientation) bool {
        return switch (self) {
            .normal, .rot90, .rot180, .rot270 => false,
            .flipped, .flipped_rot90, .flipped_rot180, .flipped_rot270 => true,
        };
    }

    pub fn toTransform(self: Orientation) TransformMatrix {
        const deg = self.degrees();
        const flip = self.isFlipped();
        const rad = @as(f64, @floatFromInt(deg)) * std.math.pi / 180.0;
        const cos_val = std.math.cos(rad);
        const sin_val = std.math.sin(rad);

        if (flip) {
            return .{
                .xx = cos_val,
                .xy = sin_val,
                .yx = sin_val,
                .yy = -cos_val,
                .x0 = if (deg == 90 or deg == 270) 1.0 else 0.0,
                .y0 = if (deg == 0 or deg == 180) 1.0 else 0.0,
            };
        }
        return .{
            .xx = cos_val,
            .xy = -sin_val,
            .yx = sin_val,
            .yy = cos_val,
            .x0 = if (deg == 270) 1.0 else 0.0,
            .y0 = if (deg == 180) 1.0 else 0.0,
        };
    }

    pub fn fromTransform(m: TransformMatrix) Orientation {
        const is_horiz = @abs(m.xx) > 0.9 and @abs(m.yy) < 0.1;
        const is_vert = @abs(m.xx) < 0.1 and @abs(m.yy) > 0.9;

        if (is_horiz and m.xy >= 0 and m.yx <= 0) {
            return if (m.xx > 0) .normal else .rot180;
        }
        if (is_horiz and m.xy <= 0 and m.yx >= 0) {
            return if (m.xx > 0) .flipped else .flipped_rot180;
        }
        if (is_vert and m.xy >= 0 and m.yx >= 0) {
            return if (m.xy > 0) .rot90 else .rot270;
        }
        if (is_vert and m.xy <= 0 and m.yx <= 0) {
            return if (m.xy < 0) .flipped_rot270 else .flipped_rot90;
        }
        return .normal;
    }

    pub fn rotate(self: Orientation, additional: Orientation) Orientation {
        const combined: u8 = (@intFromEnum(self) + @intFromEnum(additional)) & 0x7;
        return @enumFromInt(combined);
    }

    pub fn invert(self: Orientation) Orientation {
        return switch (self) {
            .normal => .normal,
            .rot90 => .rot270,
            .rot180 => .rot180,
            .rot270 => .rot90,
            .flipped => .flipped,
            .flipped_rot90 => .flipped_rot270,
            .flipped_rot180 => .flipped_rot180,
            .flipped_rot270 => .flipped_rot90,
        };
    }

    pub fn swapDimensions(self: Orientation) bool {
        return switch (self) {
            .rot90, .rot270, .flipped_rot90, .flipped_rot270 => true,
            .normal, .rot180, .flipped, .flipped_rot180 => false,
        };
    }

    pub fn transformPoint(self: Orientation, x: f64, y: f64, width: f64, height: f64) struct { x: f64, y: f64 } {
        const m = self.toTransform();
        const nx = m.xx * x + m.xy * y + m.x0 * width;
        const ny = m.yx * x + m.yy * y + m.y0 * height;
        return .{ .x = nx, .y = ny };
    }

    pub fn fromDegrees(deg: u32, flip: bool) Orientation {
        if (flip) {
            return switch (deg % 360) {
                0 => .flipped,
                90 => .flipped_rot90,
                180 => .flipped_rot180,
                270 => .flipped_rot270,
                else => .flipped,
            };
        }
        return switch (deg % 360) {
            0 => .normal,
            90 => .rot90,
            180 => .rot180,
            270 => .rot270,
            else => .normal,
        };
    }

    pub const TransformMatrix = struct {
        xx: f64,
        xy: f64,
        yx: f64,
        yy: f64,
        x0: f64,
        y0: f64,

        pub const IDENTITY = TransformMatrix{
            .xx = 1.0,
            .xy = 0.0,
            .yx = 0.0,
            .yy = 1.0,
            .x0 = 0.0,
            .y0 = 0.0,
        };

        pub fn multiply(self: TransformMatrix, other: TransformMatrix) TransformMatrix {
            return .{
                .xx = self.xx * other.xx + self.xy * other.yx,
                .xy = self.xx * other.xy + self.xy * other.yy,
                .yx = self.yx * other.xx + self.yy * other.yx,
                .yy = self.yx * other.xy + self.yy * other.yy,
                .x0 = self.xx * other.x0 + self.xy * other.y0 + self.x0,
                .y0 = self.yx * other.x0 + self.yy * other.y0 + self.y0,
            };
        }

        pub fn equals(self: TransformMatrix, other: TransformMatrix) bool {
            const epsilon = 1e-6;
            return @abs(self.xx - other.xx) < epsilon and
                @abs(self.xy - other.xy) < epsilon and
                @abs(self.yx - other.yx) < epsilon and
                @abs(self.yy - other.yy) < epsilon and
                @abs(self.x0 - other.x0) < epsilon and
                @abs(self.y0 - other.y0) < epsilon;
        }
    };
};
