const std = @import("std");

pub const PanfrostGpuId = enum(u32) {
    t600 = 0x6000,
    t620 = 0x6200,
    t720 = 0x7200,
    t760 = 0x7600,
    t820 = 0x8200,
    t860 = 0x8600,
    t880 = 0x8800,
    g31 = 0x07000061,
    g51 = 0x07000051,
    g52 = 0x07000052,
    g71 = 0x06000051,
    g72 = 0x06000052,
    g76 = 0x06000056,
    g57 = 0x09000057,

    pub fn name(self: PanfrostGpuId) []const u8 {
        return switch (self) {
            .t600 => "T600",
            .t620 => "T620",
            .t720 => "T720",
            .t760 => "T760",
            .t820 => "T820",
            .t860 => "T860",
            .t880 => "T880",
            .g31 => "G31",
            .g51 => "G51",
            .g52 => "G52",
            .g71 => "G71",
            .g72 => "G72",
            .g76 => "G76",
            .g57 => "G57",
        };
    }

    pub fn isMidgard(self: PanfrostGpuId) bool {
        return switch (self) {
            .t600, .t620, .t720, .t760, .t820, .t860, .t880 => true,
            else => false,
        };
    }

    pub fn isBifrost(self: PanfrostGpuId) bool {
        return switch (self) {
            .g31, .g51, .g52, .g71, .g72, .g76, .g57 => true,
            else => false,
        };
    }
};

pub const PanfrostDriver = struct {
    gpu_id: PanfrostGpuId,
    num_cores: u32,
    l2_size: u32,
    job_slots: u32,
    thread_max: u32,
    texture_features: u32,
    version_major: u32,
    version_minor: u32,
    device_path: [64:0]u8,
    device_path_len: usize,

    pub fn init() PanfrostDriver {
        return .{
            .gpu_id = .g31,
            .num_cores = 1,
            .l2_size = 0,
            .job_slots = 2,
            .thread_max = 0,
            .texture_features = 0,
            .version_major = 0,
            .version_minor = 0,
            .device_path = std.mem.zeroes([64:0]u8),
            .device_path_len = 0,
        };
    }

    pub fn detect(self: *PanfrostDriver) !void {
        self.gpu_id = try self.readGpuId();
        self.num_cores = try self.readCoreCount();
        self.l2_size = try self.readL2Size();
        self.thread_max = try self.readThreadMax();
        self.texture_features = try self.readTextureFeatures();
        self.detectCapabilities();
        try self.findDevicePath();
    }

    fn readGpuId(self: *PanfrostDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/class/devfreq/.*panfrost/gpu_id",
            "/sys/devices/platform/*.gpu/gpu_id",
            "/sys/devices/platform/soc/*.gpu/devfreq/*.gpu/gpu_id",
        };
        for (paths) |path| {
            if (readSysfsValue(path)) |val| return val;
        }
        return error.PanfrostGpuIdNotFound;
    }

    fn readCoreCount(self: *PanfrostDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/class/devfreq/.*panfrost/cur_freq",
            "/sys/devices/platform/*.gpu/devfreq/*.gpu/cur_freq",
        };
        for (paths) |path| {
            if (readSysfsValue(path)) |val| {
                if (val > 0) return @max(1, val / 500000);
            }
        }
        return 1;
    }

    fn readL2Size(self: *PanfrostDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/class/devfreq/.*panfrost/max_freq",
            "/sys/devices/platform/*.gpu/devfreq/*.gpu/max_freq",
        };
        for (paths) |path| {
            if (readSysfsValue(path)) |val| return val;
        }
        return 256 * 1024;
    }

    fn readThreadMax(self: *PanfrostDriver) !u32 {
        _ = self;
        const path = "/sys/module/panfrost/parameters/panfrost_job_timeout";
        if (readSysfsValue(path)) |val| {
            if (val > 0) return val;
        }
        return switch (self.gpu_id) {
            .t600, .t620, .t720, .t760 => 128,
            .t820, .t860, .t880 => 256,
            .g31, .g51 => 256,
            .g52, .g71 => 512,
            .g72, .g76, .g57 => 1024,
        };
    }

    fn readTextureFeatures(self: *PanfrostDriver) !u32 {
        _ = self;
        const path = "/sys/module/panfrost/parameters/panfrost_mali_prfcnt";
        if (readSysfsValue(path)) |val| return val;
        return 0;
    }

    fn detectCapabilities(self: *PanfrostDriver) void {
        self.version_major = if (self.gpu_id.isMidgard()) 1 else 2;
        self.version_minor = 0;
    }

    fn findDevicePath(self: *PanfrostDriver) !void {
        const dev_dri = "/dev/dri/";
        var dir = std.fs.cwd().openDir(dev_dri, .{ .iterate = true }) catch return error.OpenFailed;
        defer dir.close();
        var iter = dir.iterate();
        while (try iter.next()) |entry| {
            if (entry.kind == .character_device) {
                if (std.mem.startsWith(u8, entry.name, "renderD") or std.mem.startsWith(u8, entry.name, "card")) {
                    const path = std.fmt.bufPrintZ(&self.device_path, "/dev/dri/{s}", .{entry.name}) catch continue;
                    _ = path;
                    if (std.fs.cwd().accessAbsolute(&self.device_path, .{})) {
                        self.device_path_len = std.mem.indexOfScalar(u8, &self.device_path, 0) orelse self.device_path.len;
                        return;
                    } else |_| {}
                }
            }
        }
        return error.DevicePathNotFound;
    }

    pub fn getDevicePath(self: *const PanfrostDriver) [:0]const u8 {
        return self.device_path[0..self.device_path_len :0];
    }

    pub fn isSupported(self: *const PanfrostDriver) bool {
        return true;
    }

    pub fn getNumCores(self: *const PanfrostDriver) u32 {
        return self.num_cores;
    }

    pub fn getGpuId(self: *const PanfrostDriver) PanfrostGpuId {
        return self.gpu_id;
    }

    pub fn getCoreMask(self: *const PanfrostDriver) u32 {
        return (@as(u32, 1) << @intCast(self.num_cores)) - 1;
    }

    pub fn getMaxWorkgroupSize(self: *const PanfrostDriver) u32 {
        return if (self.gpu_id.isMidgard()) 256 else 512;
    }

    pub fn getMaxThreads(self: *const PanfrostDriver) u32 {
        return self.thread_max;
    }
};

fn readSysfsValue(path: []const u8) ?u32 {
    const content = std.fs.cwd().readFileAlloc(std.heap.page_allocator, path, 4096) catch return null;
    defer std.heap.page_allocator.free(content);
    const trimmed = std.mem.trim(u8, content, " \n\t");
    return std.fmt.parseInt(u32, trimmed, 0) catch null;
}
