const std = @import("std");

pub const DisplayMode = struct {
    width: u32,
    height: u32,
    refresh_rate_mhz: u32,
    preferred: bool,
    vrr_capable: bool,

    pub fn init(width: u32, height: u32, refresh_rate_mhz: u32, preferred: bool) DisplayMode {
        return .{
            .width = width,
            .height = height,
            .refresh_rate_mhz = refresh_rate_mhz,
            .preferred = preferred,
            .vrr_capable = false,
        };
    }

    pub fn isPreferred(self: DisplayMode) bool {
        return self.preferred;
    }

    pub fn isSimilar(self: DisplayMode, other: DisplayMode) bool {
        if (self.width != other.width or self.height != other.height) return false;
        const diff = if (self.refresh_rate_mhz > other.refresh_rate_mhz)
            self.refresh_rate_mhz - other.refresh_rate_mhz
        else
            other.refresh_rate_mhz - self.refresh_rate_mhz;
        return diff <= 1000;
    }

    pub fn matchesResolution(self: DisplayMode, width: u32, height: u32) bool {
        return self.width == width and self.height == height;
    }

    pub fn matchesRefreshRate(self: DisplayMode, rate_mhz: u32) bool {
        const diff = if (self.refresh_rate_mhz > rate_mhz)
            self.refresh_rate_mhz - rate_mhz
        else
            rate_mhz - self.refresh_rate_mhz;
        return diff <= 500;
    }

    pub fn refreshRateHz(self: DisplayMode) f64 {
        return @as(f64, @floatFromInt(self.refresh_rate_mhz)) / 1000.0;
    }

    pub fn pixelCount(self: DisplayMode) u64 {
        return @as(u64, self.width) * @as(u64, self.height);
    }

    pub fn aspectRatio(self: DisplayMode) f64 {
        if (self.height == 0) return 0;
        return @as(f64, @floatFromInt(self.width)) / @as(f64, @floatFromInt(self.height));
    }

    pub fn totalBandwidthMbit(self: DisplayMode) f64 {
        const pixels = self.pixelCount();
        const refresh = self.refreshRateHz();
        return @as(f64, @floatFromInt(pixels)) * refresh * 24.0 / 1000000.0;
    }

    pub fn compareByResolution(_: void, a: DisplayMode, b: DisplayMode) bool {
        if (a.width == b.width) {
            if (a.height == b.height) {
                return a.refresh_rate_mhz > b.refresh_rate_mhz;
            }
            return a.height > b.height;
        }
        return a.width > b.width;
    }

    pub fn compareByRefreshRate(_: void, a: DisplayMode, b: DisplayMode) bool {
        return a.refresh_rate_mhz > b.refresh_rate_mhz;
    }

    pub fn compareByPreferred(_: void, a: DisplayMode, b: DisplayMode) bool {
        if (a.preferred and !b.preferred) return true;
        if (!a.preferred and b.preferred) return false;
        return a.refresh_rate_mhz > b.refresh_rate_mhz;
    }

    pub fn format(self: DisplayMode) DisplayModeFormat {
        return .{ .mode = self };
    }

    pub const DisplayModeFormat = struct {
        mode: DisplayMode,

        pub fn format(self: DisplayModeFormat, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            try writer.print("{d}x{d}@{d:.2}Hz", .{
                self.mode.width,
                self.mode.height,
                self.mode.refreshRateHz(),
            });
        }
    };

    pub const COMMON_MODES = [_]DisplayMode{
        init(1920, 1080, 60000, false),
        init(1920, 1080, 144000, false),
        init(2560, 1440, 60000, false),
        init(2560, 1440, 144000, false),
        init(3840, 2160, 60000, true),
        init(3840, 2160, 120000, false),
        init(1280, 720, 60000, false),
        init(1024, 768, 60000, false),
        init(1600, 900, 60000, false),
        init(2560, 1080, 60000, false),
    };
};
