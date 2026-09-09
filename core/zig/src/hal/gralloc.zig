const std = @import("std");

const c = @cImport({
    @cInclude("android/hardware_buffer.h");
    @cInclude("system/graphics.h");
});

pub const BufferInfo = struct {
    width: u32,
    height: u32,
    format: u32,
    usage: u64,
    stride: u32,
    layer_count: u32,
    pixel_stride: u32,

    pub fn fromHardwareBuffer(desc: c.AHardwareBuffer_Desc) BufferInfo {
        return .{
            .width = desc.width,
            .height = desc.height,
            .format = desc.format,
            .usage = desc.usage,
            .stride = desc.stride,
            .layer_count = desc.layers,
            .pixel_stride = 4,
        };
    }

    pub fn estimateSize(self: BufferInfo) u64 {
        return @as(u64, self.stride) * @as(u64, self.height);
    }

    pub fn isRgb(self: BufferInfo) bool {
        return self.format == c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM or
            self.format == c.AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM or
            self.format == c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT;
    }

    pub fn isYuv(self: BufferInfo) bool {
        return self.format == c.AHARDWAREBUFFER_FORMAT_YCbCr_420_888;
    }

    pub fn isBlob(self: BufferInfo) bool {
        return self.format == c.AHARDWAREBUFFER_FORMAT_BLOB;
    }

    pub fn hasAlpha(self: BufferInfo) bool {
        return self.format == c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM or
            self.format == c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT;
    }
};

pub const GrallocHal = struct {
    alloc_device: ?*anyopaque,
    free_device: ?*anyopaque,
    version: u32,
    is_loaded: bool,

    pub const Error = error{
        NotLoaded,
        AllocationFailed,
        FreeFailed,
        InvalidBuffer,
        UnsupportedFormat,
        UnsupportedUsage,
    };

    pub fn init() GrallocHal {
        return .{
            .alloc_device = null,
            .free_device = null,
            .version = 0,
            .is_loaded = false,
        };
    }

    pub fn load(self: *GrallocHal) !void {
        self.alloc_device = @ptrFromInt(1);
        self.free_device = @ptrFromInt(2);
        self.version = 4;
        self.is_loaded = true;
    }

    pub fn deinit(self: *GrallocHal) void {
        self.alloc_device = null;
        self.free_device = null;
        self.is_loaded = false;
        self.version = 0;
    }

    pub fn allocate(self: *GrallocHal, width: u32, height: u32, format: u32, usage: u64) !AllocResult {
        if (!self.is_loaded) return Error.NotLoaded;
        if (width == 0 or height == 0) return Error.InvalidBuffer;

        const stride = self.calculateStride(width, format);
        const size = @as(u64, stride) * @as(u64, height);

        return .{
            .handle = self.alloc_device,
            .info = .{
                .width = width,
                .height = height,
                .format = format,
                .usage = usage,
                .stride = stride,
                .layer_count = 1,
                .pixel_stride = self.getPixelStride(format),
            },
            .size = size,
        };
    }

    pub fn free(self: *GrallocHal, handle: ?*anyopaque) !void {
        if (!self.is_loaded) return Error.NotLoaded;
        if (handle == null) return Error.InvalidBuffer;
        _ = handle;
    }

    pub fn lock(self: *GrallocHal, handle: ?*anyopaque, usage: u64, x: i32, y: i32, width: i32, height: i32) !LockResult {
        if (!self.is_loaded) return Error.NotLoaded;
        _ = handle;
        _ = usage;
        return .{
            .vaddr = @ptrFromInt(0x1000),
            .pitch = width * 4,
        };
    }

    pub fn unlock(self: *GrallocHal, handle: ?*anyopaque) !void {
        if (!self.is_loaded) return Error.NotLoaded;
        _ = handle;
    }

    pub fn isSupported(self: *const GrallocHal, width: u32, height: u32, format: u32, usage: u64) bool {
        if (!self.is_loaded) return false;
        if (width == 0 or height == 0) return false;
        _ = format;
        _ = usage;
        return true;
    }

    pub fn getDeviceInfo(self: *const GrallocHal) DeviceInfo {
        return .{
            .max_buffer_size = 1024 * 1024 * 1024,
            .max_layer_count = 32,
            .max_pixel_stride = 4,
            .supported_formats = &.{ c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM, c.AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM, c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT, c.AHARDWAREBUFFER_FORMAT_BLOB, c.AHARDWAREBUFFER_FORMAT_YCbCr_420_888 },
        };
    }

    pub const AllocResult = struct {
        handle: ?*anyopaque,
        info: BufferInfo,
        size: u64,
    };

    pub const LockResult = struct {
        vaddr: *anyopaque,
        pitch: u32,
    };

    pub const DeviceInfo = struct {
        max_buffer_size: u64,
        max_layer_count: u32,
        max_pixel_stride: u32,
        supported_formats: []const u32,
    };

    fn calculateStride(self: *GrallocHal, width: u32, format: u32) u32 {
        _ = self;
        const bpp = switch (format) {
            c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM, c.AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM => 4,
            c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT => 8,
            c.AHARDWAREBUFFER_FORMAT_BLOB => 1,
            c.AHARDWAREBUFFER_FORMAT_YCbCr_420_888 => 1,
            else => 4,
        };
        const alignment: u32 = 64;
        const raw = width * bpp;
        return (raw + alignment - 1) & ~(alignment - 1);
    }

    fn getPixelStride(self: *GrallocHal, format: u32) u32 {
        _ = self;
        return switch (format) {
            c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM, c.AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM => 4,
            c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT => 8,
            c.AHARDWAREBUFFER_FORMAT_BLOB => 1,
            c.AHARDWAREBUFFER_FORMAT_YCbCr_420_888 => 1,
            else => 4,
        };
    }
};
