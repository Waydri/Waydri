const std = @import("std");

pub const DmaBufAttrs = struct {
    fd: std.posix.fd_t,
    offset: u64,
    size: u64,
    modifiers: u64,
    pitch: u32,
    width: u32,
    height: u32,
    format: u32,

    pub fn isValid(self: DmaBufAttrs) bool {
        return self.fd >= 0 and self.size > 0 and self.width > 0 and self.height > 0;
    }

    pub fn totalSize(self: DmaBufAttrs) u64 {
        return self.size;
    }
};

pub const DmaBufExporter = struct {
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) DmaBufExporter {
        return .{ .allocator = allocator };
    }

    pub fn exportBuffer(self: *DmaBufExporter, attrs: DmaBufAttrs) !DmaBufHandle {
        _ = self;
        if (!attrs.isValid()) return error.InvalidAttributes;
        return .{
            .fd = attrs.fd,
            .size = attrs.size,
            .offset = attrs.offset,
            .attrs = attrs,
        };
    }

    pub fn destroyHandle(self: *DmaBufExporter, handle: *DmaBufHandle) void {
        _ = self;
        if (handle.fd >= 0) {
            std.posix.close(handle.fd) catch {};
            handle.fd = -1;
        }
    }
};

pub const DmaBufImporter = struct {
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) DmaBufImporter {
        return .{ .allocator = allocator };
    }

    pub fn importBuffer(self: *DmaBufImporter, handle: DmaBufHandle) !DmaBufAttrs {
        _ = self;
        if (handle.fd < 0) return error.InvalidFd;
        return handle.attrs;
    }
};

pub const DmaBufSync = struct {
    pub fn startCpuRead(fd: std.posix.fd_t) !void {
        var sync = std.mem.zeroes(SyncObj);
        sync.flags = .{ .read = true };
        _ = std.io.linux.ioctl(fd, SYNC_IOC_WAIT, @intCast(@intFromPtr(&sync)));
    }

    pub fn endCpuRead(fd: std.posix.fd_t) !void {
        var sync = std.mem.zeroes(SyncObj);
        sync.flags = .{ .read = true };
        _ = std.io.linux.ioctl(fd, SYNC_IOC_SIGNAL, @intCast(@intFromPtr(&sync)));
    }

    pub fn startCpuWrite(fd: std.posix.fd_t) !void {
        var sync = std.mem.zeroes(SyncObj);
        sync.flags = .{ .write = true };
        _ = std.io.linux.ioctl(fd, SYNC_IOC_WAIT, @intCast(@intFromPtr(&sync)));
    }

    pub fn endCpuWrite(fd: std.posix.fd_t) !void {
        var sync = std.mem.zeroes(SyncObj);
        sync.flags = .{ .write = true };
        _ = std.io.linux.ioctl(fd, SYNC_IOC_SIGNAL, @intCast(@intFromPtr(&sync)));
    }

    pub fn waitAll(fd: std.posix.fd_t, timeout_ns: i64) !void {
        var sync = std.mem.zeroes(SyncObj);
        sync.flags = .{ .read = true, .write = true };
        sync.timeout_ns = timeout_ns;
        _ = std.io.linux.ioctl(fd, SYNC_IOC_WAIT, @intCast(@intFromPtr(&sync)));
    }

    pub fn merge(fd1: std.posix.fd_t, fd2: std.posix.fd_t) !std.posix.fd_t {
        _ = fd1;
        _ = fd2;
        const new_fd = std.posix.open("/dev/null", .{ .ACCMODE = .RDONLY }, 0) catch return error.MergeFailed;
        return new_fd;
    }

    const SyncObj = extern struct {
        flags: packed struct(u32) {
            read: bool = false,
            write: bool = false,
            _padding: u30 = 0,
        } = .{},
        timeout_ns: i64 = 0,
    };

    const SYNC_IOC_WAIT: u32 = 0x40086406;
    const SYNC_IOC_MERGE: u32 = 0x40046403;
    const SYNC_IOC_SIGNAL: u32 = 0x40086407;
};

pub const DmaBufAllocator = struct {
    exporter: DmaBufExporter,
    importer: DmaBufImporter,

    pub fn init(allocator: std.mem.Allocator) DmaBufAllocator {
        return .{
            .exporter = DmaBufExporter.init(allocator),
            .importer = DmaBufImporter.init(allocator),
        };
    }

    pub fn export(self: *DmaBufAllocator, attrs: DmaBufAttrs) !DmaBufHandle {
        return self.exporter.exportBuffer(attrs);
    }

    pub fn import(self: *DmaBufAllocator, handle: DmaBufHandle) !DmaBufAttrs {
        return self.importer.importBuffer(handle);
    }

    pub fn sync(self: *DmaBufAllocator, fd: std.posix.fd_t, start: bool) !void {
        if (start) {
            try DmaBufSync.startCpuRead(fd);
        } else {
            try DmaBufSync.endCpuRead(fd);
        }
    }

    pub const DmaBufHandle = struct {
        fd: std.posix.fd_t,
        size: u64,
        offset: u64,
        attrs: DmaBufAttrs,

        pub fn isValid(self: DmaBufHandle) bool {
            return self.fd >= 0 and self.size > 0;
        }

        pub fn close(self: *DmaBufHandle) void {
            if (self.fd >= 0) {
                std.posix.close(self.fd) catch {};
                self.fd = -1;
            }
        }
    };
};
