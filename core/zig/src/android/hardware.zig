const std = @import("std");

const c = @cImport({
    @cInclude("android/hardware_buffer.h");
    @cInclude("android/hardware_buffer_jni.h");
    @cInclude("system/graphics.h");
});

pub const BufferFormat = enum(u32) {
    rgba8 = 1,
    rgbx8 = 2,
    rgba_fp16 = 3,
    blob = 4,
    yuv = 5,
    undefined = 0,

    pub fn toNative(self: BufferFormat) u32 {
        return switch (self) {
            .rgba8 => c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM,
            .rgbx8 => c.AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM,
            .rgba_fp16 => c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT,
            .blob => c.AHARDWAREBUFFER_FORMAT_BLOB,
            .yuv => c.AHARDWAREBUFFER_FORMAT_YCbCr_420_888,
            .undefined => 0,
        };
    }

    pub fn fromNative(native: u32) BufferFormat {
        if (native == c.AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM) return .rgba8;
        if (native == c.AHARDWAREBUFFER_FORMAT_R8G8B8X8_UNORM) return .rgbx8;
        if (native == c.AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT) return .rgba_fp16;
        if (native == c.AHARDWAREBUFFER_FORMAT_BLOB) return .blob;
        if (native == c.AHARDWAREBUFFER_FORMAT_YCbCr_420_888) return .yuv;
        return .undefined;
    }

    pub fn bytesPerPixel(self: BufferFormat) u32 {
        return switch (self) {
            .rgba8 => 4,
            .rgbx8 => 4,
            .rgba_fp16 => 8,
            .blob => 1,
            .yuv => 1,
            .undefined => 0,
        };
    }
};

pub const BufferUsage = packed struct(u64) {
    cpu_read: bool = false,
    cpu_write: bool = false,
    gpu_texture: bool = false,
    gpu_render_target: bool = false,
    compositor_overlay: bool = false,
    video_decode: bool = false,
    video_encode: bool = false,
    camera: bool = false,
    _padding: u56 = 0,

    pub const CPU_READ: BufferUsage = .{ .cpu_read = true };
    pub const CPU_WRITE: BufferUsage = .{ .cpu_write = true };
    pub const GPU_TEXTURE: BufferUsage = .{ .gpu_texture = true };
    pub const GPU_RENDER_TARGET: BufferUsage = .{ .gpu_render_target = true };
    pub const COMPOSITOR_OVERLAY: BufferUsage = .{ .compositor_overlay = true };
    pub const VIDEO_DECODE: BufferUsage = .{ .video_decode = true };
    pub const VIDEO_ENCODE: BufferUsage = .{ .video_encode = true };
    pub const CAMERA: BufferUsage = .{ .camera = true };

    pub fn toNative(self: BufferUsage) u64 {
        return @as(u64, @bitCast(self));
    }
};

pub const HardwareBuffer = struct {
    handle: ?*c.AHardwareBuffer,
    desc: c.AHardwareBuffer_Desc,
    format: BufferFormat,
    usage: BufferUsage,
    width: u32,
    height: u32,

    pub fn allocate(width: u32, height: u32, format: BufferFormat, usage: BufferUsage) !HardwareBuffer {
        var desc = std.mem.zeroes(c.AHardwareBuffer_Desc);
        desc.width = width;
        desc.height = height;
        desc.format = format.toNative();
        desc.usage = usage.toNative();

        var handle: ?*c.AHardwareBuffer = null;
        const result = c.AHardwareBuffer_allocate(&desc, &handle);
        if (result != 0) return error.AllocationFailed;

        return .{
            .handle = handle,
            .desc = desc,
            .format = format,
            .usage = usage,
            .width = width,
            .height = height,
        };
    }

    pub fn lock(self: *HardwareBuffer) !LockResult {
        const handle = self.handle orelse return error.InvalidBuffer;
        var vaddr: ?*anyopaque = null;
        const result = c.AHardwareBuffer_lock(handle, &vaddr, -1, null, null);
        if (result != 0) return error.LockFailed;
        return .{
            .vaddr = vaddr orelse return error.LockFailed,
            .pitch = self.desc.stride * self.format.bytesPerPixel(),
        };
    }

    pub fn lockRegion(self: *HardwareBuffer, x: u32, y: u32, width: u32, height: u32) !LockResult {
        const handle = self.handle orelse return error.InvalidBuffer;
        var vaddr: ?*anyopaque = null;
        const rect = c.AHardwareBuffer_Rect{
            .left = @intCast(x),
            .top = @intCast(y),
            .right = @intCast(x + width),
            .bottom = @intCast(y + height),
        };
        const result = c.AHardwareBuffer_lock(handle, &vaddr, -1, @ptrCast(&rect), null);
        if (result != 0) return error.LockFailed;
        return .{
            .vaddr = vaddr orelse return error.LockFailed,
            .pitch = self.desc.stride * self.format.bytesPerPixel(),
        };
    }

    pub fn unlock(self: *HardwareBuffer) !void {
        const handle = self.handle orelse return error.InvalidBuffer;
        const result = c.AHardwareBuffer_unlock(handle, null);
        if (result != 0) return error.UnlockFailed;
    }

    pub fn release(self: *HardwareBuffer) void {
        if (self.handle) |handle| {
            c.AHardwareBuffer_decreaseRefcount(handle);
        }
    }

    pub fn getDescription(self: *const HardwareBuffer) BufferDesc {
        return .{
            .width = self.width,
            .height = self.height,
            .format = self.format,
            .usage = self.usage,
            .stride = self.desc.stride,
        };
    }

    pub fn retain(self: *HardwareBuffer) !void {
        const handle = self.handle orelse return error.InvalidBuffer;
        const result = c.AHardwareBuffer_increaseRefcount(handle);
        if (result != 0) return error.RetainFailed;
    }

    pub fn isSupported(width: u32, height: u32, format: BufferFormat, usage: BufferUsage) bool {
        var desc = std.mem.zeroes(c.AHardwareBuffer_Desc);
        desc.width = width;
        desc.height = height;
        desc.format = format.toNative();
        desc.usage = usage.toNative();
        return c.AHardwareBuffer_isSupported(&desc) == 0;
    }

    pub fn getId(self: *const HardwareBuffer) u32 {
        const handle = self.handle orelse return 0;
        return c.AHardwareBuffer_getId(handle);
    }

    pub fn getNativeHandle(self: *const HardwareBuffer) ?*const c.AHardwareBuffer {
        return self.handle;
    }
};

pub const LockResult = struct {
    vaddr: *anyopaque,
    pitch: u32,
};

pub const BufferDesc = struct {
    width: u32,
    height: u32,
    format: BufferFormat,
    usage: BufferUsage,
    stride: u32,
};
