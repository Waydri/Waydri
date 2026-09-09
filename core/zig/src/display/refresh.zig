const std = @import("std");

pub const RefreshRate = struct {
    numerator: u64,
    denominator: u64,

    pub fn init(numerator: u64, denominator: u64) RefreshRate {
        if (denominator == 0) return .{ .numerator = 0, .denominator = 1 };
        const g = gcd(numerator, denominator);
        return .{
            .numerator = numerator / g,
            .denominator = denominator / g,
        };
    }

    pub fn fromHz(hz: f64) RefreshRate {
        const scaled = hz * 1000.0;
        const num: u64 = @intFromFloat(@round(scaled));
        return init(num, 1000);
    }

    pub fn toHz(self: RefreshRate) f64 {
        if (self.denominator == 0) return 0;
        return @as(f64, @floatFromInt(self.numerator)) / @as(f64, @floatFromInt(self.denominator));
    }

    pub fn toMhz(self: RefreshRate) u32 {
        const hz = self.toHz();
        return @intFromFloat(@round(hz * 1000.0));
    }

    pub fn toUhz(self: RefreshRate) u64 {
        return self.numerator * 1000000 / self.denominator;
    }

    pub fn isInteger(self: RefreshRate) bool {
        return self.numerator % self.denominator == 0;
    }

    pub fn isVrr(self: RefreshRate) bool {
        const hz = self.toHz();
        const rounded: u64 = @intFromFloat(@round(hz));
        const hz_u64: u64 = @intFromFloat(hz);
        return rounded != hz_u64;
    }

    pub fn equals(self: RefreshRate, other: RefreshRate) bool {
        return self.numerator * other.denominator == other.numerator * self.denominator;
    }

    pub fn isClose(self: RefreshRate, other: RefreshRate, tolerance_mhz: u32) bool {
        const diff_mhz = if (self.toMhz() > other.toMhz())
            self.toMhz() - other.toMhz()
        else
            other.toMhz() - self.toMhz();
        return diff_mhz <= tolerance_mhz;
    }

    pub fn compareByRate(_: void, a: RefreshRate, b: RefreshRate) bool {
        return a.numerator * b.denominator > b.numerator * a.denominator;
    }

    pub fn add(self: RefreshRate, other: RefreshRate) RefreshRate {
        const new_num = self.numerator * other.denominator + other.numerator * self.denominator;
        const new_den = self.denominator * other.denominator;
        return init(new_num, new_den);
    }

    pub fn subtract(self: RefreshRate, other: RefreshRate) RefreshRate {
        const new_num = self.numerator * other.denominator - other.numerator * self.denominator;
        const new_den = self.denominator * other.denominator;
        return init(new_num, new_den);
    }

    pub fn multiply(self: RefreshRate, scalar: u64) RefreshRate {
        return init(self.numerator * scalar, self.denominator);
    }

    pub fn invert(self: RefreshRate) RefreshRate {
        if (self.numerator == 0) return .{ .numerator = 0, .denominator = 1 };
        return .{ .numerator = self.denominator, .denominator = self.numerator };
    }

    pub fn frameDurationNs(self: RefreshRate) u64 {
        if (self.numerator == 0) return 0;
        return self.denominator * 1000000000 / self.numerator;
    }

    pub fn format(self: RefreshRate) RefreshRateFormat {
        return .{ .rate = self };
    }

    pub const RefreshRateFormat = struct {
        rate: RefreshRate,

        pub fn format(self: RefreshRateFormat, comptime _: []const u8, _: std.fmt.FormatOptions, writer: anytype) !void {
            const hz = self.rate.toHz();
            const rounded = @round(hz * 100.0) / 100.0;
            try writer.print("{d:.2} Hz ({d}/{d})", .{ rounded, self.rate.numerator, self.rate.numerator });
        }
    };

    pub const COMMON_RATES = [_]RefreshRate{
        init(60000, 1000),
        init(144000, 1000),
        init(240000, 1000),
        init(300000, 1000),
        init(120000, 1000),
        init(90000, 1000),
        init(50000, 1000),
        init(25000, 1000),
        init(23976, 1000),
        init(29970, 1000),
        init(59940, 1000),
        init(119880, 1000),
    };

    fn gcd(a: u64, b: u64) u64 {
        var x = a;
        var y = b;
        while (y != 0) {
            const temp = y;
            y = x % y;
            x = temp;
        }
        return if (x == 0) 1 else x;
    }
};
