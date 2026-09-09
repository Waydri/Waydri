const std = @import("std");

const c = @cImport({
    @cInclude("wayland-server-core.h");
});

pub const ShmPool = struct {
    fd: std.posix.fd_t,
    data: ?[*]u8,
    size: usize,
    ref_count: u32,

    pub fn create(size: usize) !ShmPool {
        if (size == 0) return error.InvalidSize;
        const fd = std.posix.memfd_create("waydri-shm", .{ .CLOEXEC = true }) catch return error.CreateFailed;
        const result = std.posix.ftruncate(fd, @intCast(size));
        if (result) |_| {} else |_| {
            std.posix.close(fd);
            return error.TruncateFailed;
        }
        const mapped = std.posix.mmap(null, size, .{ .READ = true, .WRITE = true }, .SHARED, fd, 0) catch {
            std.posix.close(fd);
            return error.MmapFailed;
        };
        return .{
            .fd = fd,
            .data = mapped.ptr,
            .size = size,
            .ref_count = 1,
        };
    }

    pub fn destroy(self: *ShmPool) void {
        if (self.ref_count == 0) return;
        self.ref_count -= 1;
        if (self.ref_count == 0) {
            if (self.data) |data| {
                const slice: []u8 = @ptrCast(@alignCast(data));
                std.posix.munmap(slice[0..self.size]);
                self.data = null;
            }
            if (self.fd >= 0) {
                std.posix.close(self.fd) catch {};
                self.fd = -1;
            }
        }
    }

    pub fn resize(self: *ShmPool, new_size: usize) !void {
        if (new_size == 0) return error.InvalidSize;
        if (new_size == self.size) return;
        if (self.data) |data| {
            const old_slice: []u8 = @ptrCast(@alignCast(data));
            std.posix.munmap(old_slice[0..self.size]);
            self.data = null;
        }
        const result = std.posix.ftruncate(self.fd, @intCast(new_size));
        if (result) |_| {} else |_| return error.TruncateFailed;
        const mapped = std.posix.mmap(null, new_size, .{ .READ = true, .WRITE = true }, .SHARED, self.fd, 0) catch return error.MmapFailed;
        self.data = mapped.ptr;
        self.size = new_size;
    }

    pub fn getData(self: *ShmPool) ?[]u8 {
        if (self.data) |data| {
            return data[0..self.size];
        }
        return null;
    }

    pub fn getFd(self: *const ShmPool) std.posix.fd_t {
        return self.fd;
    }

    pub fn getSize(self: *const ShmPool) usize {
        return self.size;
    }

    pub fn retain(self: *ShmPool) void {
        self.ref_count += 1;
    }

    pub fn getOffset(self: *const ShmPool, offset: usize, length: usize) ?[]u8 {
        if (offset + length > self.size) return null;
        if (self.data) |data| {
            return data[offset..][0..length];
        }
        return null;
    }

    pub fn writeAt(self: *ShmPool, offset: usize, data: []const u8) !void {
        if (offset + data.len > self.size) return error.InvalidSize;
        if (self.data) |pool_data| {
            const dest: []u8 = @ptrCast(@alignCast(pool_data));
            @memcpy(dest[offset..][0..data.len], data);
        }
    }

    pub fn readAt(self: *const ShmPool, offset: usize, length: usize) ?[]const u8 {
        if (offset + length > self.size) return null;
        if (self.data) |data| {
            return data[offset..][0..length];
        }
        return null;
    }

    pub fn fillZero(self: *ShmPool, offset: usize, length: usize) !void {
        if (offset + length > self.size) return error.InvalidSize;
        if (self.data) |data| {
            const slice: []u8 = @ptrCast(@alignCast(data));
            @memset(slice[offset..][0..length], 0);
        }
    }
};

pub const ShmBuffer = struct {
    pool: *ShmPool,
    offset: u32,
    width: u32,
    height: u32,
    stride: u32,
    format: u32,
    wl_buffer: ?*anyopaque,

    pub const SHM_FORMAT_ARGB8888: u32 = 0;
    pub const SHM_FORMAT_XRGB8888: u32 = 1;
    pub const SHM_FORMAT_RGBA8888: u32 = 0x34324152;
    pub const SHM_FORMAT_RGB565: u32 = 0x36314752;

    pub fn init(pool: *ShmPool, offset: u32, width: u32, height: u32, format: u32) ShmBuffer {
        const bpp: u32 = switch (format) {
            SHM_FORMAT_ARGB8888, SHM_FORMAT_XRGB8888, SHM_FORMAT_RGBA8888 => 4,
            SHM_FORMAT_RGB565 => 2,
            else => 4,
        };
        const stride = (width * bpp + 3) & ~@as(u32, 3);
        return .{
            .pool = pool,
            .offset = offset,
            .width = width,
            .height = height,
            .stride = stride,
            .format = format,
            .wl_buffer = null,
        };
    }

    pub fn getData(self: *const ShmBuffer) ?[]u8 {
        return self.pool.getOffset(self.offset, self.stride * self.height);
    }

    pub fn writeToPixel(self: *const ShmBuffer, x: u32, y: u32, pixel: u32) !void {
        if (x >= self.width or y >= self.height) return error.InvalidCoordinates;
        const bpp: u32 = switch (self.format) {
            SHM_FORMAT_ARGB8888, SHM_FORMAT_XRGB8888, SHM_FORMAT_RGBA8888 => 4,
            SHM_FORMAT_RGB565 => 2,
            else => 4,
        };
        const offset = self.offset + y * self.stride + x * bpp;
        const data = self.pool.getOffset(offset, bpp) orelse return error.InvalidOffset;
        const pixel_bytes = std.mem.toBytes(pixel);
        const write_len = @min(bpp, @as(u32, @intCast(pixel_bytes.len)));
        @memcpy(data[0..write_len], pixel_bytes[0..write_len]);
    }

    pub fn clear(self: *const ShmBuffer, color: u32) !void {
        if (self.getData()) |data| {
            const bpp: u32 = switch (self.format) {
                SHM_FORMAT_ARGB8888, SHM_FORMAT_XRGB8888, SHM_FORMAT_RGBA8888 => 4,
                SHM_FORMAT_RGB565 => 2,
                else => 4,
            };
            var y: u32 = 0;
            while (y < self.height) : (y += 1) {
                var x: u32 = 0;
                while (x < self.width) : (x += 1) {
                    const pixel_offset = (y * self.stride + x * bpp);
                    if (pixel_offset + bpp <= data.len) {
                        const pixel_bytes = std.mem.toBytes(color);
                        @memcpy(data[pixel_offset..][0..bpp], pixel_bytes[0..bpp]);
                    }
                }
            }
        }
    }

    pub fn attachToWlBuffer(self: *ShmBuffer, wl_buffer: ?*anyopaque) void {
        self.wl_buffer = wl_buffer;
    }

    pub fn getStride(self: *const ShmBuffer) u32 {
        return self.stride;
    }

    pub fn getSize(self: *const ShmBuffer) u64 {
        return @as(u64, self.stride) * @as(u64, self.height);
    }
};
