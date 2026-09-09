const std = @import("std");

const android = @import("android");
const display = @import("display");
const driver = @import("driver");
const gpu = @import("gpu");
const hal = @import("hal");
const input = @import("input");
const memory = @import("memory");
const sync = @import("sync");

pub const CoreName = "waydri-core";

pub const CoreState = struct {
    allocator: std.mem.Allocator,
    gpu_backend: gpu.GpuBackend = .software,
    driver_type: ?driver.DriverType = null,
    has_egl: bool = false,
    has_vulkan: bool = false,
    has_hwc: bool = false,
    has_drm: bool = false,
    has_gralloc: bool = false,
    input_ready: bool = false,
    sync_ready: bool = false,
    buffer_ready: bool = false,
    initialized: bool = false,

    pub fn init(allocator: std.mem.Allocator) CoreState {
        return .{ .allocator = allocator };
    }

    pub fn deinit(self: *CoreState) void {
        _ = self;
    }
};

pub const Core = struct {
    state: CoreState,
    sync: sync.event.EventLoop = undefined,
    buffer: memory.allocator.BufferAllocator = undefined,

    pub fn create(allocator: std.mem.Allocator) !Core {
        var state = CoreState.init(allocator);
        state.buffer_ready = true;
        return .{ .state = state };
    }

    pub fn destroy(self: *Core) void {
        self.state.deinit();
    }

    pub fn name(self: *const Core) []const u8 {
        _ = self;
        return CoreName;
    }

    pub fn isReady(self: *const Core) bool {
        return self.state.initialized;
    }

    pub fn backendName(self: *const Core) []const u8 {
        return self.state.gpu_backend.name();
    }

    pub fn supportsHdr(self: *const Core) bool {
        return self.state.gpu_backend.supportsHdr();
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    var core = try Core.create(allocator);
    defer core.destroy();

    const stderr = std.io.getStdErr().writer();
    try stderr.print("{s} initialized: backend={s}\n", .{ core.name(), core.backendName() });
}
