const std = @import("std");

pub const UsageFlags = packed struct(u64) {
    gpu_render: bool = false,
    cpu_read: bool = false,
    cpu_write: bool = false,
    video: bool = false,
    camera: bool = false,
    composit: bool = false,
    protected: bool = false,
    cursor: bool = false,
    _padding: u56 = 0,

    pub const GPU_RENDER: UsageFlags = .{ .gpu_render = true };
    pub const CPU_READ: UsageFlags = .{ .cpu_read = true };
    pub const CPU_WRITE: UsageFlags = .{ .cpu_write = true };
    pub const VIDEO: UsageFlags = .{ .video = true };
    pub const CAMERA: UsageFlags = .{ .camera = true };
    pub const COMPOSIT: UsageFlags = .{ .composit = true };
    pub const PROTECTED: UsageFlags = .{ .protected = true };
    pub const CURSOR: UsageFlags = .{ .cursor = true };

    pub fn toNative(self: UsageFlags) u64 {
        return @as(u64, @bitCast(self));
    }

    pub fn isGpuOnly(self: UsageFlags) bool {
        return self.gpu_render and !self.cpu_read and !self.cpu_write;
    }

    pub fn isCpuOnly(self: UsageFlags) bool {
        return (self.cpu_read or self.cpu_write) and !self.gpu_render;
    }

    pub fn isHwComposer(self: UsageFlags) bool {
        return self.composit;
    }
};

pub const BufferDescriptor = struct {
    width: u32,
    height: u32,
    layer_count: u32,
    format: BufferFormat,
    usage: UsageFlags,
    stride: u32,
    size: u64,
    id: u32,
};

pub const BufferFormat = enum(u32) {
    rgba8888 = 1,
    rgbx8888 = 2,
    rgba_fp16 = 3,
    blob = 4,
    yuv420_888 = 5,
    ycbcr_p010 = 6,
    rgba_1010102 = 7,
    undefined = 0,

    pub fn bytesPerPixel(self: BufferFormat) u32 {
        return switch (self) {
            .rgba8888 => 4,
            .rgbx8888 => 4,
            .rgba_fp16 => 8,
            .blob => 1,
            .yuv420_888 => 3,
            .ycbcr_p010 => 4,
            .rgba_1010102 => 4,
            .undefined => 0,
        };
    }

    pub fn hasAlpha(self: BufferFormat) bool {
        return switch (self) {
            .rgba8888, .rgba_fp16, .rgba_1010102 => true,
            .rgbx8888, .blob, .yuv420_888, .ycbcr_p010, .undefined => false,
        };
    }

    pub fn isYuv(self: BufferFormat) bool {
        return switch (self) {
            .yuv420_888, .ycbcr_p010 => true,
            else => false,
        };
    }
};

pub const GrallocAllocator = struct {
    allocator_impl: ?*anyopaque,
    version: u32,
    is_loaded: bool,

    pub fn init() GrallocAllocator {
        return .{
            .allocator_impl = null,
            .version = 0,
            .is_loaded = false,
        };
    }

    pub fn load(self: *GrallocAllocator) !void {
        self.allocator_impl = @ptrFromInt(1);
        self.version = 4;
        self.is_loaded = true;
    }

    pub fn release(self: *GrallocAllocator) void {
        self.allocator_impl = null;
        self.is_loaded = false;
        self.version = 0;
    }

    pub fn allocate(self: *GrallocAllocator, descriptor: BufferDescriptor) !BufferHandle {
        if (!self.is_loaded) return error.NotLoaded;
        const stride = calculateStride(descriptor.width, descriptor.format);
        const size = @as(u64, stride) * @as(u64, descriptor.height);
        return .{
            .id = @intCast(@intFromPtr(self.allocator_impl)),
            .handle = self.allocator_impl,
            .stride = stride,
            .size = size,
        };
    }

    pub fn free(self: *GrallocAllocator, handle: BufferHandle) !void {
        _ = self;
        _ = handle;
    }

    pub fn isSupported(self: *const GrallocAllocator, descriptor: BufferDescriptor) bool {
        _ = self;
        return descriptor.width > 0 and descriptor.height > 0 and descriptor.format != .undefined;
    }

    pub fn getSupportedFormats(self: *const GrallocAllocator) []const BufferFormat {
        return &.{ .rgba8888, .rgbx8888, .rgba_fp16, .rgba_1010102, .yuv420_888, .ycbcr_p010 };
    }

    pub fn getStreamUsage(self: *const GrallocAllocator) UsageFlags {
        return .{ .gpu_render = true, .cpu_read = true };
    }

    pub fn getCursorUsage(self: *const GrallocAllocator) UsageFlags {
        return .{ .cursor = true, .cpu_write = true };
    }

    pub const BufferHandle = struct {
        id: u32,
        handle: ?*anyopaque,
        stride: u32,
        size: u64,

        pub fn isValid(self: BufferHandle) bool {
            return self.handle != null;
        }
    };

    fn calculateStride(width: u32, format: BufferFormat) u32 {
        const bpp = format.bytesPerPixel();
        const alignment: u32 = 64;
        const raw = width * bpp;
        return (raw + alignment - 1) & ~(alignment - 1);
    }
};
