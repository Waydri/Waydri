pub const adreno = @import("adreno.zig");
pub const mali = @import("mali.zig");
pub const freedreno = @import("freedreno.zig");
pub const panfrost = @import("panfrost.zig");

pub const DriverType = enum {
    adreno,
    mali,
    freedreno,
    panfrost,

    pub fn name(self: DriverType) []const u8 {
        return switch (self) {
            .adreno => "Adreno",
            .mali => "Mali",
            .freedreno => "Freedreno",
            .panfrost => "Panfrost",
        };
    }

    pub fn vendor(self: DriverType) []const u8 {
        return switch (self) {
            .adreno => "Qualcomm",
            .mali => "ARM",
            .freedreno => "Qualcomm (open)",
            .panfrost => "ARM (open)",
        };
    }

    pub fn isQualcomm(self: DriverType) bool {
        return self == .adreno or self == .freedreno;
    }

    pub fn isArm(self: DriverType) bool {
        return self == .mali or self == .panfrost;
    }

    pub fn isOpenSource(self: DriverType) bool {
        return self == .freedreno or self == .panfrost;
    }
};
