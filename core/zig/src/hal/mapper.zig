const std = @import("std");

const c = @cImport({
    @cInclude("android/hardware_buffer.h");
});

pub const MapperHal = struct {
    mapper: ?*anyopaque,
    version: u32,
    is_loaded: bool,

    pub const Error = error{
        NotLoaded,
        ImportFailed,
        RetainFailed,
        ReleaseFailed,
        LockFailed,
        UnlockFailed,
        InvalidBuffer,
        TransportSizeFailed,
    };

    pub const TransportSize = struct {
        total_size: u64,
        metadata_size: u64,
    };

    pub const BufferMetadata = struct {
        id: u32,
        width: u32,
        height: u32,
        format: u32,
        usage: u64,
        stride: u32,
        layer_count: u32,
        pixel_stride: u32,
    };

    pub fn init() MapperHal {
        return .{
            .mapper = null,
            .version = 0,
            .is_loaded = false,
        };
    }

    pub fn load(self: *MapperHal) !void {
        self.mapper = @ptrFromInt(1);
        self.version = 4;
        self.is_loaded = true;
    }

    pub fn deinit(self: *MapperHal) void {
        self.mapper = null;
        self.is_loaded = false;
        self.version = 0;
    }

    pub fn importBuffer(self: *MapperHal, handle: ?*const anyopaque) !u32 {
        if (!self.is_loaded) return Error.NotLoaded;
        if (handle == null) return Error.InvalidBuffer;
        return @intCast(@intFromPtr(handle));
    }

    pub fn retainBuffer(self: *MapperHal, buffer_id: u32) !void {
        if (!self.is_loaded) return Error.NotLoaded;
        if (buffer_id == 0) return Error.InvalidBuffer;
        _ = buffer_id;
    }

    pub fn releaseBuffer(self: *MapperHal, buffer_id: u32) !void {
        if (!self.is_loaded) return Error.NotLoaded;
        if (buffer_id == 0) return Error.InvalidBuffer;
        _ = buffer_id;
    }

    pub fn lock(self: *MapperHal, buffer_id: u32, usage: u64, x: i32, y: i32, width: i32, height: i32, fence: c_int) !LockResult {
        if (!self.is_loaded) return Error.NotLoaded;
        if (buffer_id == 0) return Error.InvalidBuffer;
        _ = usage;
        _ = x;
        _ = y;
        _ = width;
        _ = height;
        _ = fence;
        return .{
            .vaddr = @ptrFromInt(0x2000),
            .stride = width * 4,
        };
    }

    pub fn unlock(self: *MapperHal, buffer_id: u32, fence: c_int) !void {
        if (!self.is_loaded) return Error.NotLoaded;
        if (buffer_id == 0) return Error.InvalidBuffer;
        _ = fence;
    }

    pub fn getTransportSize(self: *MapperHal, buffer_id: u32) !TransportSize {
        if (!self.is_loaded) return Error.NotLoaded;
        if (buffer_id == 0) return Error.InvalidBuffer;
        return .{
            .total_size = @sizeOf(c.AHardwareBuffer_Desc) + 256,
            .metadata_size = 64,
        };
    }

    pub fn getMetadata(self: *MapperHal, buffer_id: u32) !BufferMetadata {
        if (!self.is_loaded) return Error.NotLoaded;
        if (buffer_id == 0) return Error.InvalidBuffer;
        return .{
            .id = buffer_id,
            .width = 0,
            .height = 0,
            .format = 0,
            .usage = 0,
            .stride = 0,
            .layer_count = 1,
            .pixel_stride = 4,
        };
    }

    pub fn isLoaded(self: *const MapperHal) bool {
        return self.is_loaded;
    }

    pub fn getVersion(self: *const MapperHal) u32 {
        return self.version;
    }

    pub fn isSupported(self: *const MapperHal, buffer_id: u32) bool {
        return self.is_loaded and buffer_id != 0;
    }

    pub const LockResult = struct {
        vaddr: *anyopaque,
        stride: u32,
    };
};
