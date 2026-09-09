const std = @import("std");

const c = @cImport({
    @cInclude("libdrm/drm.h");
    @cInclude("xf86drm.h");
});

pub const AdrenoFeatures = packed struct(u32) {
    has_600_plus: bool = false,
    has_a6xx_texture_fetch: bool = false,
    has_render_doc: bool = false,
    has_a6xx: bool = false,
    has_a7xx: bool = false,
    has_ubwc: bool = false,
    has_lrz: bool = false,
    has_zap: bool = false,
    has_gmem: bool = false,
    has_sparse: bool = false,
    has_wave64: bool = false,
    has_d3d12: bool = false,
    _padding: u20 = 0,
};

pub const AdrenoGpuId = enum(u32) {
    a200 = 0x02000000,
    a205 = 0x02050000,
    a220 = 0x02200000,
    a225 = 0x02250000,
    a300 = 0x03000000,
    a305 = 0x03050000,
    a306 = 0x03060000,
    a320 = 0x03200000,
    a330 = 0x03300000,
    a350 = 0x03500000,
    a420 = 0x04200000,
    a430 = 0x04300000,
    a450 = 0x04500000,
    a505 = 0x05050000,
    a506 = 0x05060000,
    a510 = 0x05100000,
    a512 = 0x05120000,
    a515 = 0x05150000,
    a530 = 0x05300000,
    a540 = 0x05400000,
    a610 = 0x06100000,
    a612 = 0x06120000,
    a615 = 0x06150000,
    a618 = 0x06180000,
    a620 = 0x06200000,
    a630 = 0x06300000,
    a635 = 0x06350000,
    a640 = 0x06400000,
    a650 = 0x06500000,
    a660 = 0x06600000,
    a662 = 0x06620000,
    a668 = 0x06680000,
    a670 = 0x06700000,
    a680 = 0x06800000,
    a690 = 0x06900000,
    a702 = 0x07020000,
    a703 = 0x07030000,
    a707 = 0x07070000,
    a708 = 0x07080000,
    a710 = 0x07100000,
    a712 = 0x07120000,
    a715 = 0x07150000,
    a718 = 0x07180000,
    a720 = 0x07200000,
    a723 = 0x07230000,
    a725 = 0x07250000,
    a730 = 0x07300000,
    a735 = 0x07350000,
    a740 = 0x07400000,
    a750 = 0x07500000,

    pub fn name(self: AdrenoGpuId) []const u8 {
        const id = @intFromEnum(self);
        const major = (id >> 24) & 0xFF;
        const minor = (id >> 16) & 0xFF;
        const patch = (id >> 8) & 0xFF;
        if (patch != 0) {
            return std.fmt.allocPrint(std.heap.page_allocator, "Adreno {d}.{d}.{d}", .{ major, minor, patch }) catch "Adreno";
        }
        return std.fmt.allocPrint(std.heap.page_allocator, "Adreno {d}.{d}", .{ major, minor }) catch "Adreno";
    }

    pub fn majorVersion(self: AdrenoGpuId) u32 {
        return (@intFromEnum(self) >> 24) & 0xFF;
    }
};

pub const AdrenoDriver = struct {
    device_fd: std.posix.fd_t,
    gpu_id: AdrenoGpuId,
    features: AdrenoFeatures,
    chip_id: u32,
    gmem_size: u32,
    vram_size: u64,

    pub const DevicePath = "/dev/dri/card0";

    pub fn init(device_fd: std.posix.fd_t) AdrenoDriver {
        return .{
            .device_fd = device_fd,
            .gpu_id = .a610,
            .features = .{},
            .chip_id = 0,
            .gmem_size = 0,
            .vram_size = 0,
        };
    }

    pub fn detect(self: *AdrenoDriver) !void {
        self.chip_id = try self.regRead(0x400);
        self.gmem_size = try self.regRead(0x48);
        self.gpu_id = @enumFromInt(self.chip_id & 0xFF000000);

        const major = self.gpu_id.majorVersion();
        self.features.has_600_plus = major >= 6;
        self.features.has_a6xx = major >= 6 and major < 7;
        self.features.has_a7xx = major >= 7;
        self.features.has_a6xx_texture_fetch = major >= 6;
        self.features.has_ubwc = major >= 5;
        self.features.has_lrz = major >= 5;
        self.features.has_zap = major >= 6;
        self.features.has_gmem = major >= 5;
        self.features.has_sparse = major >= 6;
        self.features.has_wave64 = major >= 7;
        self.features.has_d3d12 = major >= 7;
    }

    pub fn regRead(self: *AdrenoDriver, offset: u32) !u32 {
        var req = std.mem.zeroes(c.drm_iowrite32_ptr);
        req.offset = @intCast(offset);
        var value: u32 = 0;
        req.data = @ptrCast(&value);
        const result = std.io.linux.ioctl(self.device_fd, c.DRM_IOCTL_ADRENO_GETPARAM, @intCast(@intFromPtr(&req)));
        if (result < 0) return error.RegReadFailed;
        return value;
    }

    pub fn getGpuId(self: *const AdrenoDriver) AdrenoGpuId {
        return self.gpu_id;
    }

    pub fn getChipId(self: *const AdrenoDriver) u32 {
        return self.chip_id;
    }

    pub fn getGmemSize(self: *const AdrenoDriver) u32 {
        return self.gmem_size;
    }

    pub fn hasFeature(self: *const AdrenoDriver, feature: AdrenoFeatures) bool {
        return @as(u32, @bitCast(self.features & feature)) != 0;
    }

    pub fn isSupported(self: *const AdrenoDriver) bool {
        const major = self.gpu_id.majorVersion();
        return major >= 3;
    }

    pub fn getDevicePath(self: *const AdrenoDriver) [:0]const u8 {
        return DevicePath;
    }

    pub fn getShaderOffsetAlign(self: *const AdrenoDriver) u32 {
        if (self.features.has_a7xx) return 128;
        if (self.features.has_a6xx) return 64;
        return 32;
    }

    pub fn getMaxTextureSize(self: *const AdrenoDriver) u32 {
        if (self.features.has_a7xx) return 16384;
        if (self.features.has_a6xx) return 16384;
        return 8192;
    }

    pub fn getMaxViewportDims(self: *const AdrenoDriver) u32 {
        if (self.features.has_a7xx) return 16384;
        if (self.features.has_a6xx) return 16384;
        return 4096;
    }
};
