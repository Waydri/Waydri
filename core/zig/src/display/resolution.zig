const std = @import("std");

pub const Resolution = struct {
    width: u32,
    height: u32,
    scale_factor: u32,

    pub fn init(width: u32, height: u32) Resolution {
        return .{
            .width = width,
            .height = height,
            .scale_factor = 1,
        };
    }

    pub fn initWithScale(width: u32, height: u32, scale_factor: u32) Resolution {
        return .{
            .width = width,
            .height = height,
            .scale_factor = if (scale_factor == 0) 1 else scale_factor,
        };
    }

    pub fn scale_factor(self: Resolution) u32 {
        return self.scale_factor;
    }

    pub fn scaled(self: Resolution) ScaledResolution {
        return .{
            .physical_width = self.width,
            .physical_height = self.height,
            .logical_width = self.width / self.scale_factor,
            .logical_height = self.height / self.scale_factor,
            .scale_factor = self.scale_factor,
        };
    }

    pub fn pixelCount(self: Resolution) u64 {
        return @as(u64, self.width) * @as(u64, self.height);
    }

    pub fn aspectRatio(self: Resolution) f64 {
        if (self.height == 0) return 0;
        return @as(f64, @floatFromInt(self.width)) / @as(f64, @floatFromInt(self.height));
    }

    pub fn isWidescreen(self: Resolution) bool {
        const ratio = self.aspectRatio();
        return ratio >= 1.6;
    }

    pub fn isUltrawide(self: Resolution) bool {
        const ratio = self.aspectRatio();
        return ratio >= 2.1;
    }

    pub fn is4K(self: Resolution) bool {
        return self.width >= 3840 and self.height >= 2160;
    }

    pub fn isPortrait(self: Resolution) bool {
        return self.height > self.width;
    }

    pub fn isLandscape(self: Resolution) bool {
        return self.width > self.height;
    }

    pub fn isSquare(self: Resolution) bool {
        return self.width == self.height;
    }

    pub fn rotated(self: Resolution) Resolution {
        return .{
            .width = self.height,
            .height = self.width,
            .scale_factor = self.scale_factor,
        };
    }

    pub fn withScale(self: Resolution, new_scale: u32) Resolution {
        return .{
            .width = self.width,
            .height = self.height,
            .scale_factor = if (new_scale == 0) 1 else new_scale,
        };
    }

    pub fn matches(self: Resolution, other: Resolution) bool {
        return self.width == other.width and self.height == other.height;
    }

    pub fn contains(self: Resolution, other: Resolution) bool {
        return self.width >= other.width and self.height >= other.height;
    }

    pub fn maxDim(self: Resolution) u32 {
        return @max(self.width, self.height);
    }

    pub fn minDim(self: Resolution) u32 {
        return @min(self.width, self.height);
    }

    pub fn diagonalInches(self: Resolution, dpi: f64) f64 {
        if (dpi == 0) return 0;
        const w = @as(f64, @floatFromInt(self.width)) / dpi;
        const h = @as(f64, @floatFromInt(self.height)) / dpi;
        return std.math.sqrt(w * w + h * h);
    }

    pub fn compareBySize(_: void, a: Resolution, b: Resolution) bool {
        return a.pixelCount() > b.pixelCount();
    }

    pub fn compareByWidth(_: void, a: Resolution, b: Resolution) bool {
        return a.width > b.width;
    }

    pub fn format(self: Resolution) ResolutionFormat {
        return .{ .res = self };
    }

    pub const ResolutionFormat = struct {
        res: Resolution,

        pub fn format(self: ResolutionFormat, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            try writer.print("{d}x{d}", .{ self.res.width, self.res.height });
            if (self.res.scale_factor > 1) {
                try writer.print("@{d}x", .{self.res.scale_factor});
            }
        }
    };

    pub const ScaledResolution = struct {
        physical_width: u32,
        physical_height: u32,
        logical_width: u32,
        logical_height: u32,
        scale_factor: u32,

        pub fn isValid(self: ScaledResolution) bool {
            return self.logical_width > 0 and self.logical_height > 0;
        }
    };

    pub const COMMON_RESOLUTIONS = [_]Resolution{
        init(3840, 2160),
        init(2560, 1440),
        init(1920, 1080),
        init(1600, 900),
        init(1366, 768),
        init(1280, 720),
        init(1024, 768),
        init(2560, 1080),
        init(3440, 1440),
        init(5120, 2880),
    };
};
