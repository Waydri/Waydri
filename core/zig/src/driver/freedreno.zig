const std = @import("std");

pub const FreedrenoDevice = enum(u32) {
    a2xx = 0x02000000,
    a3xx = 0x03000000,
    a4xx = 0x04000000,
    a5xx = 0x05000000,
    a6xx = 0x06000000,
    a7xx = 0x07000000,

    pub fn name(self: FreedrenoDevice) []const u8 {
        return switch (self) {
            .a2xx => "Adreno 2xx",
            .a3xx => "Adreno 3xx",
            .a4xx => "Adreno 4xx",
            .a5xx => "Adreno 5xx",
            .a6xx => "Adreno 6xx",
            .a7xx => "Adreno 7xx",
        };
    }

    pub fn majorVersion(self: FreedrenoDevice) u32 {
        return (@intFromEnum(self) >> 24) & 0xFF;
    }
};

pub const FreedrenoDriver = struct {
    device_id: u32,
    gpu_id: u32,
    device: FreedrenoDevice,
    chip_id: u32,
    gmem_size: u32,
    gmem_base: u32,
    va_start: u64,
    va_size: u64,
    device_path: [64:0]u8,
    device_path_len: usize,

    pub fn init() FreedrenoDriver {
        return .{
            .device_id = 0,
            .gpu_id = 0,
            .device = .a6xx,
            .chip_id = 0,
            .gmem_size = 0,
            .gmem_base = 0,
            .va_start = 0,
            .va_size = 0,
            .device_path = std.mem.zeroes([64:0]u8),
            .device_path_len = 0,
        };
    }

    pub fn detect(self: *FreedrenoDriver) !void {
        self.device_id = try self.readDeviceId();
        self.gpu_id = try self.readGpuId();
        self.device = @enumFromInt(self.device_id);
        self.gmem_size = try self.readGmemSize();
        self.chip_id = self.device_id | (self.gpu_id & 0x00FF0000);
        try self.findDevicePath();
    }

    fn readDeviceId(self: *FreedrenoDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/class/kgsl/kgsl-3d0/gpu_id",
            "/sys/devices/platform/soc/*.qcom,kgsl-3d0/kgsl-3d0/gpu_id",
            "/sys/class/kgsl/kgsl-3d0/device_id",
        };
        for (paths) |path| {
            if (readSysfsValue(path)) |val| return val;
        }
        return error.DeviceIdNotFound;
    }

    fn readGpuId(self: *FreedrenoDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/class/kgsl/kgsl-3d0/gpu_id",
            "/sys/class/kgsl/kgsl-3d0/gpu_model",
            "/sys/devices/platform/soc/*.qcom,kgsl-3d0/kgsl-3d0/gpu_id",
        };
        for (paths) |path| {
            if (readSysfsValue(path)) |val| return val;
        }
        return error.GpuIdNotFound;
    }

    fn readGmemSize(self: *FreedrenoDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/class/kgsl/kgsl-3d0/gmem_size",
            "/sys/devices/platform/soc/*.qcom,kgsl-3d0/kgsl-3d0/gmem_size",
        };
        for (paths) |path| {
            if (readSysfsValue(path)) |val| return val;
        }
        return error.GmemSizeNotFound;
    }

    fn findDevicePath(self: *FreedrenoDriver) !void {
        const prefixes = [_][]const u8{ "/dev/dri/card", "/dev/dri/renderD" };
        for (prefixes) |prefix| {
            var i: u32 = 0;
            while (i < 16) : (i += 1) {
                const path = std.fmt.bufPrintZ(&self.device_path, "{s}{d}", .{ prefix, i }) catch continue;
                _ = path;
                if (std.fs.cwd().accessAbsolute(&self.device_path, .{})) {
                    self.device_path_len = std.mem.indexOfScalar(u8, &self.device_path, 0) orelse self.device_path.len;
                    return;
                } else |_| {}
            }
        }
        return error.DevicePathNotFound;
    }

    pub fn getDevicePath(self: *const FreedrenoDriver) [:0]const u8 {
        return self.device_path[0..self.device_path_len :0];
    }

    pub fn isSupported(self: *const FreedrenoDriver) bool {
        const major = self.device.majorVersion();
        return major >= 3;
    }

    pub fn getGmemSize(self: *const FreedrenoDriver) u32 {
        return self.gmem_size;
    }

    pub fn getChipId(self: *const FreedrenoDriver) u32 {
        return self.chip_id;
    }

    pub fn hasUBWC(self: *const FreedrenoDriver) bool {
        return self.device.majorVersion() >= 5;
    }

    pub fn hasLRZ(self: *const FreedrenoDriver) bool {
        return self.device.majorVersion() >= 5;
    }

    pub fn hasA6xxSPFS(self: *const FreedrenoDriver) bool {
        return self.device.majorVersion() >= 6;
    }

    pub fn getMaxIndexBufferSize(self: *const FreedrenoDriver) u64 {
        return switch (self.device) {
            .a2xx, .a3xx => 16 * 1024 * 1024,
            .a4xx, .a5xx => 128 * 1024 * 1024,
            .a6xx, .a7xx => 256 * 1024 * 1024,
        };
    }

    pub fn getVABlockSize(self: *const FreedrenoDriver) u64 {
        return switch (self.device) {
            .a2xx, .a3xx => 256 * 1024 * 1024,
            .a4xx, .a5xx => 4 * 1024 * 1024 * 1024,
            .a6xx, .a7xx => 4 * 1024 * 1024 * 1024,
        };
    }
};

fn readSysfsValue(path: []const u8) ?u32 {
    var buf: [4096]u8 = undefined;
    var dir = std.fs.cwd().openDir(std.fs.path.dirname(path) orelse return null, .{}) catch return null;
    defer dir.close();
    const filename = std.fs.path.basename(path);
    const content = dir.readFileAlloc(std.heap.page_allocator, filename, 4096) catch return null;
    defer std.heap.page_allocator.free(content);
    const trimmed = std.mem.trim(u8, content, " \n\t");
    return std.fmt.parseInt(u32, trimmed, 0) catch null;
}
