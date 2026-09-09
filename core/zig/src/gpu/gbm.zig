const std = @import("std");

const c = @cImport({
    @cInclude("GBM/gbm.h");
});

pub const GbmFormat = enum(u32) {
    rgb888 = c.GBM_FORMAT_RGB888,
    bgr888 = c.GBM_FORMAT_BGR888,
    rgba8888 = c.GBM_FORMAT_RGBA8888,
    bgra8888 = c.GBM_FORMAT_BGRA8888,
    argb8888 = c.GBM_FORMAT_ARGB8888,
    abgr8888 = c.GBM_FORMAT_ABGR8888,
    xrgb8888 = c.GBM_FORMAT_XRGB8888,
    xbgr8888 = c.GBM_FORMAT_XBGR8888,
    rgb565 = c.GBM_FORMAT_RGB565,
    rgba_f16 = c.GBM_FORMAT_RGBA16F,

    pub fn bpp(self: GbmFormat) u32 {
        return switch (self) {
            .rgb888, .bgr888 => 24,
            .rgba8888, .bgra8888, .argb8888, .abgr8888, .xrgb8888, .xbgr8888 => 32,
            .rgb565 => 16,
            .rgba_f16 => 64,
        };
    }

    pub fn hasAlpha(self: GbmFormat) bool {
        return switch (self) {
            .argb8888, .abgr8888, .rgba8888, .bgra8888, .rgba_f16 => true,
            else => false,
        };
    }
};

pub const GbmBufferObject = struct {
    bo: ?*c.gbm_bo,
    width: u32,
    height: u32,
    format: GbmFormat,
    stride: u32,
    handle: u32,
    size: u64,

    pub fn lock(self: *GbmBufferObject) ![]u8 {
        const bo = self.bo orelse return error.InvalidBuffer;
        const ptr = c.gbm_bo_map(bo, 0, 0, self.width, self.height, c.GBM_BO_TRANSFER_READ_WRITE);
        if (ptr == null) return error.MapFailed;
        const byte_len = @as(usize, self.stride) * @as(usize, self.height);
        return @as([*]u8, @ptrCast(ptr))[0..byte_len];
    }

    pub fn unlock(self: *GbmBufferObject) void {
        if (self.bo) |bo| {
            c.gbm_bo_unmap(bo);
        }
    }

    pub fn getHandle(self: *const GbmBufferObject) u32 {
        return self.handle;
    }

    pub fn getStride(self: *const GbmBufferObject) u32 {
        return self.stride;
    }

    pub fn getFormat(self: *const GbmBufferObject) GbmFormat {
        return self.format;
    }

    pub fn getSize(self: *const GbmBufferObject) u64 {
        return self.size;
    }

    pub fn isFailed(self: *const GbmBufferObject) bool {
        return self.bo == null;
    }
};

pub const GbmSurface = struct {
    surface: ?*c.gbm_surface,
    width: u32,
    height: u32,
    format: GbmFormat,
    flags: u32,

    pub fn init(device: *c.gbm_device, width: u32, height: u32, format: GbmFormat, flags: u32) !GbmSurface {
        const surface = c.gbm_surface_create(
            device,
            width,
            height,
            @intFromEnum(format),
            flags,
        );
        if (surface == null) return error.SurfaceCreationFailed;
        return .{
            .surface = surface,
            .width = width,
            .height = height,
            .format = format,
            .flags = flags,
        };
    }

    pub fn deinit(self: *GbmSurface) void {
        if (self.surface) |surface| {
            c.gbm_surface_destroy(surface);
            self.surface = null;
        }
    }

    pub fn lockFrontBuffer(self: *GbmSurface) !GbmBufferObject {
        const surface = self.surface orelse return error.InvalidSurface;
        const bo = c.gbm_surface_lock_front_buffer(surface);
        if (bo == null) return error.NoFrontBuffer;

        const handle = c.gbm_bo_get_handle(bo);
        const stride = c.gbm_bo_get_stride(bo);
        const size = c.gbm_bo_get_size(bo);

        return .{
            .bo = bo,
            .width = self.width,
            .height = self.height,
            .format = self.format,
            .stride = @intCast(handle.u32),
            .handle = handle.u32,
            .size = size,
        };
    }

    pub fn unlockFrontBuffer(self: *GbmSurface, buffer: GbmBufferObject) !void {
        const surface = self.surface orelse return error.InvalidSurface;
        if (buffer.bo) |bo| {
            const result = c.gbm_surface_release_buffer(surface, bo);
            if (result == 0) return error.ReleaseFailed;
        }
    }

    pub fn hasFreeBuffer(self: *const GbmSurface) bool {
        if (self.surface) |surface| {
            return c.gbm_surface_has_free_buffers(surface) != 0;
        }
        return false;
    }

    pub fn getWidth(self: *const GbmSurface) u32 {
        return self.width;
    }

    pub fn getHeight(self: *const GbmSurface) u32 {
        return self.height;
    }

    pub fn getFormat(self: *const GbmSurface) GbmFormat {
        return self.format;
    }

    pub const SCANOUT: u32 = c.GBM_BO_USE_SCANOUT;
    pub const RENDERING: u32 = c.GBM_BO_USE_RENDERING;
    pub const CURSOR: u32 = c.GBM_BO_USE_CURSOR;
    pub const LINEAR: u32 = c.GBM_BO_USE_LINEAR;
};

pub const GbmDevice = struct {
    device: ?*c.gbm_device,
    fd: std.posix.fd_t,
    backend_name: ?[:0]const u8,

    pub const DevicePath = "/dev/dri/card0";

    pub fn open(path: [:0]const u8) !GbmDevice {
        const fd = std.posix.open(path, .{ .ACCMODE = .RDWR, .CLOEXEC = true }, 0) catch return error.OpenFailed;
        const dev = c.gbm_create_device(@intCast(fd));
        if (dev == null) {
            std.posix.close(fd);
            return error.DeviceCreationFailed;
        }
        const name = c.gbm_device_get_backend_name(dev);
        return .{
            .device = dev,
            .fd = fd,
            .backend_name = if (name) |n| std.mem.span(n) else null,
        };
    }

    pub fn close(self: *GbmDevice) void {
        if (self.device) |dev| {
            c.gbm_device_destroy(dev);
            self.device = null;
        }
        std.posix.close(self.fd);
    }

    pub fn createSurface(self: *GbmDevice, width: u32, height: u32, format: GbmFormat, flags: u32) !GbmSurface {
        const dev = self.device orelse return error.InvalidDevice;
        return GbmSurface.init(dev, width, height, format, flags);
    }

    pub fn createBufferObject(self: *GbmDevice, width: u32, height: u32, format: GbmFormat, flags: u32) !GbmBufferObject {
        const dev = self.device orelse return error.InvalidDevice;
        const bo = c.gbm_bo_create(dev, width, height, @intFromEnum(format), flags);
        if (bo == null) return error.BufferCreationFailed;

        const handle = c.gbm_bo_get_handle(bo);
        const stride = c.gbm_bo_get_stride(bo);
        const size = c.gbm_bo_get_size(bo);

        return .{
            .bo = bo,
            .width = width,
            .height = height,
            .format = format,
            .stride = stride,
            .handle = handle.u32,
            .size = size,
        };
    }

    pub fn destroyBufferObject(self: *GbmDevice, buffer: *GbmBufferObject) void {
        if (buffer.bo) |bo| {
            c.gbm_bo_destroy(self.device orelse return);
            buffer.bo = null;
        }
    }

    pub fn getFd(self: *const GbmDevice) std.posix.fd_t {
        return self.fd;
    }

    pub fn getDevice(self: *const GbmDevice) ?*c.gbm_device {
        return self.device;
    }

    pub fn getBackendName(self: *const GbmDevice) ?[:0]const u8 {
        return self.backend_name;
    }

    pub fn isFormatSupported(self: *const GbmDevice, format: GbmFormat, usage: u32) bool {
        const dev = self.device orelse return false;
        return c.gbm_device_is_format_supported(dev, @intFromEnum(format), usage) != 0;
    }

    pub fn getModifiersForFormat(self: *const GbmDevice, format: GbmFormat, usage: u32, modifiers: ?[]u64) !u32 {
        const dev = self.device orelse return error.InvalidDevice;
        const mod_ptr: ?[*]u64 = if (modifiers) |m| m.ptr else null;
        const count = c.gbm_bo_get_modifier.?(dev, @intFromEnum(format), usage, mod_ptr);
        return @intCast(count);
    }

    pub const DEFAULT_FORMAT: GbmFormat = .rgba8888;
};
