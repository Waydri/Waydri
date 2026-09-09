const std = @import("std");

const c = @cImport({
    @cInclude("android/native_window.h");
    @cInclude("android/window.h");
});

pub const BufferTransform = enum(u32) {
    identity = 0,
    rot_90 = 1,
    rot_180 = 2,
    rot_270 = 3,
    flip = 4,
    flip_rot_90 = 5,
    flip_rot_180 = 6,
    flip_rot_270 = 7,
};

pub const Surface = struct {
    window: ?*c.ANativeWindow,
    width: u32,
    height: u32,
    format: u32,
    buffer_count: u32,
    min_undequeued: u32,

    pub fn wrap(window: ?*c.ANativeWindow) Surface {
        const w = if (window) |win| @as(u32, @intCast(c.ANativeWindow_getWidth(win))) else 0;
        const h = if (window) |win| @as(u32, @intCast(c.ANativeWindow_getHeight(win))) else 0;
        return .{
            .window = window,
            .width = w,
            .height = h,
            .format = 0,
            .buffer_count = 2,
            .min_undequeued = 0,
        };
    }

    pub fn getWidth(self: *const Surface) u32 {
        return self.width;
    }

    pub fn getHeight(self: *const Surface) u32 {
        return self.height;
    }

    pub fn getDimensions(self: *const Surface) struct { width: u32, height: u32 } {
        return .{ .width = self.width, .height = self.height };
    }

    pub fn setBuffersGeometry(self: *Surface, width: i32, height: i32, format: u32) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_setBuffersGeometry(window, width, height, format);
        if (result != 0) return error.SetGeometryFailed;
        if (width > 0) self.width = @intCast(width);
        if (height > 0) self.height = @intCast(height);
        self.format = format;
    }

    pub fn setBuffersTransform(self: *Surface, transform: BufferTransform) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_setBuffersTransform(window, @intFromEnum(transform));
        if (result != 0) return error.SetTransformFailed;
    }

    pub fn setBufferCount(self: *Surface, count: u32) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_setBufferCount(window, count);
        if (result != 0) return error.SetBufferCountFailed;
        self.buffer_count = count;
    }

    pub fn setUsage(self: *Surface, usage: u64) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_setUsage(window, usage);
        if (result != 0) return error.SetUsageFailed;
    }

    pub fn dequeueBuffer(self: *Surface) !DequeuedBuffer {
        const window = self.window orelse return error.NoWindow;
        var buf: ?*c.ANativeWindowBuffer = null;
        var fence_fd: c_int = -1;
        const result = c.ANativeWindow_dequeueBuffer(window, &buf, &fence_fd);
        if (result != 0) return error.DequeueFailed;
        return .{
            .buffer = buf orelse return error.NullBuffer,
            .fence_fd = fence_fd,
        };
    }

    pub fn queueBuffer(self: *Surface, buffer: *c.ANativeWindowBuffer, fence_fd: c_int) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_queueBuffer(window, buffer, fence_fd);
        if (result != 0) return error.QueueFailed;
    }

    pub fn cancelBuffer(self: *Surface, buffer: *c.ANativeWindowBuffer, fence_fd: c_int) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_cancelBuffer(window, buffer, fence_fd);
        if (result != 0) return error.CancelFailed;
    }

    pub fn query(self: *const Surface, query: c_int) i32 {
        const window = self.window orelse return -1;
        return c.ANativeWindow_query(window, query, null);
    }

    pub fn perform(self: *Surface, operation: c_int, argument: i32) !void {
        const window = self.window orelse return error.NoWindow;
        const result = c.ANativeWindow_perform(window, operation, argument);
        if (result != 0) return error.PerformFailed;
    }

    pub fn getNativeHandle(self: *const Surface) ?*c.ANativeWindow {
        return self.window;
    }

    pub fn isAvailable(self: *const Surface) bool {
        return self.window != null;
    }

    pub fn refreshDimensions(self: *Surface) void {
        if (self.window) |window| {
            self.width = @intCast(c.ANativeWindow_getWidth(window));
            self.height = @intCast(c.ANativeWindow_getHeight(window));
        }
    }
};

pub const DequeuedBuffer = struct {
    buffer: *c.ANativeWindowBuffer,
    fence_fd: c_int,
};

pub const Queries = struct {
    pub const WIDTH: c_int = c.ANATIVEWINDOW_QUERY_WIDTH;
    pub const HEIGHT: c_int = c.ANATIVEWINDOW_QUERY_HEIGHT;
    pub const MIN_UNDEQUEUED_BUFFERS: c_int = c.ANATIVEWINDOW_QUERY_MIN_UNDEQUEUED_BUFFERS;
    pub const BUFFER_STATE: c_int = c.ANATIVEWINDOW_QUERY_BUFFER_STATE;
    pub const NATIVE_WINDOW_FORMAT: c_int = c.ANATIVEWINDOW_QUERY_FORMAT;
};

pub const PerformOps = struct {
    pub const SET_USAGE: c_int = c.ANATIVEWINDOW_SET_USAGE;
    pub const SET_BUFFERS_GEOMETRY: c_int = c.ANATIVEWINDOW_SET_BUFFERS_GEOMETRY;
    pub const SET_BUFFERS_DIMENSIONS: c_int = c.ANATIVEWINDOW_SET_BUFFERS_DIMENSIONS;
    pub const SET_BUFFERS_FORMAT: c_int = c.ANATIVEWINDOW_SET_BUFFERS_FORMAT;
    pub const SET_BUFFERS_COUNT: c_int = c.ANATIVEWINDOW_SET_BUFFERS_COUNT;
    pub const SET_TRANSFORM: c_int = c.ANATIVEWINDOW_SET_TRANSFORM;
    pub const SET_SCALING_MODE: c_int = c.ANATIVEWINDOW_SET_SCALING_MODE;
};
