const std = @import("std");

pub const EventFd = struct {
    fd: std.posix.fd_t,

    pub fn create(initial_count: u32, flags: EventFdFlags) !EventFd {
        const fd = std.posix.eventfd(initial_count, .{ .NONBLOCK = flags.nonblock, .SEMAPHORE = flags.semaphore }) catch return error.CreateFailed;
        return .{ .fd = fd };
    }

    pub fn write(self: EventFd, count: u64) !void {
        const buf = std.mem.toBytes(count);
        _ = std.posix.write(self.fd, &buf) catch return error.WriteFailed;
    }

    pub fn read(self: EventFd) !u64 {
        var buf: [@sizeOf(u64)]u8 = undefined;
        const n = std.posix.read(self.fd, &buf) catch return error.ReadFailed;
        if (n < @sizeOf(u64)) return error.IncompleteRead;
        return std.mem.bytesToValue(u64, &buf);
    }

    pub fn close(self: EventFd) void {
        std.posix.close(self.fd) catch {};
    }

    pub fn getFd(self: *const EventFd) std.posix.fd_t {
        return self.fd;
    }

    pub fn isReadable(self: *const EventFd) bool {
        var poll_fd = std.posix.pollfd{
            .fd = self.fd,
            .events = std.posix.POLL.IN,
            .revents = 0,
        };
        const result = std.posix.poll(&poll_fd, 0);
        if (result) |_| {
            return (poll_fd.revents & std.posix.POLL.IN) != 0;
        } else |_| {
            return false;
        }
    }

    pub const EventFdFlags = packed struct(u8) {
        nonblock: bool = false,
        semaphore: bool = false,
        _padding: u6 = 0,
    };
};

pub const EventLoop = struct {
    poll_fds: std.ArrayList(std.posix.pollfd),
    callbacks: std.ArrayList(?EventCallback),
    user_data: std.ArrayList(?*anyopaque),
    allocator: std.mem.Allocator,
    running: bool,
    timeout_ms: i32,

    pub const EventCallback = *const fn (std.posix.fd_t, u32, ?*anyopaque) void;

    pub const EVENTS = struct {
        pub const IN: u32 = std.posix.POLL.IN;
        pub const OUT: u32 = std.posix.POLL.OUT;
        pub const ERR: u32 = std.posix.POLL.ERR;
        pub const HUP: u32 = std.posix.POLL.HUP;
    };

    pub fn init(allocator: std.mem.Allocator) EventLoop {
        return .{
            .poll_fds = std.ArrayList(std.posix.pollfd).init(allocator),
            .callbacks = std.ArrayList(?EventCallback).init(allocator),
            .user_data = std.ArrayList(?*anyopaque).init(allocator),
            .allocator = allocator,
            .running = false,
            .timeout_ms = -1,
        };
    }

    pub fn deinit(self: *EventLoop) void {
        self.poll_fds.deinit();
        self.callbacks.deinit();
        self.user_data.deinit();
    }

    pub fn addFd(self: *EventLoop, fd: std.posix.fd_t, events: u32, callback: ?EventCallback, data: ?*anyopaque) !usize {
        const idx = self.poll_fds.items.len;
        try self.poll_fds.append(.{
            .fd = fd,
            .events = @intCast(events),
            .revents = 0,
        });
        try self.callbacks.append(callback);
        try self.user_data.append(data);
        return idx;
    }

    pub fn removeFd(self: *EventLoop, index: usize) !void {
        if (index >= self.poll_fds.items.len) return error.InvalidIndex;
        _ = self.poll_fds.orderedRemove(index);
        _ = self.callbacks.orderedRemove(index);
        _ = self.user_data.orderedRemove(index);
    }

    pub fn modifyFd(self: *EventLoop, index: usize, events: u32) !void {
        if (index >= self.poll_fds.items.len) return error.InvalidIndex;
        self.poll_fds.items[index].events = @intCast(events);
    }

    pub fn setTimeout(self: *EventLoop, timeout_ms: i32) void {
        self.timeout_ms = timeout_ms;
    }

    pub fn poll(self: *EventLoop) !usize {
        if (self.poll_fds.items.len == 0) return 0;
        const result = std.posix.poll(self.poll_fds.items, self.timeout_ms);
        var count: usize = 0;
        if (result) |n| {
            var i: usize = 0;
            while (i < self.poll_fds.items.len and i < n) : (i += 1) {
                const revents = self.poll_fds.items[i].revents;
                if (revents != 0) {
                    if (self.callbacks.items[i]) |cb| {
                        cb(self.poll_fds.items[i].fd, @intCast(revents), self.user_data.items[i]);
                    }
                    count += 1;
                }
            }
        } else |_| {}
        return count;
    }

    pub fn run(self: *EventLoop) !void {
        self.running = true;
        while (self.running) {
            _ = try self.poll();
        }
    }

    pub fn stop(self: *EventLoop) void {
        self.running = false;
    }

    pub fn getCount(self: *const EventLoop) usize {
        return self.poll_fds.items.len;
    }

    pub fn isRunning(self: *const EventLoop) bool {
        return self.running;
    }
};
