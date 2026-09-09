const std = @import("std");

pub const Fence = struct {
    fd: std.posix.fd_t,
    signaled: bool,
    merge_count: u32,

    pub fn createSignaled() !Fence {
        const fd = std.posix.memfd_create("waydri-fence", .{ .CLOEXEC = true }) catch return error.CreateFailed;
        const val: u64 = 1;
        _ = std.posix.write(fd, std.mem.toBytes(val)) catch {
            std.posix.close(fd);
            return error.WriteFailed;
        };
        return .{
            .fd = fd,
            .signaled = true,
            .merge_count = 1,
        };
    }

    pub fn createUnsignaled() !Fence {
        const fd = std.posix.memfd_create("waydri-fence", .{ .CLOEXEC = true }) catch return error.CreateFailed;
        return .{
            .fd = fd,
            .signaled = false,
            .merge_count = 1,
        };
    }

    pub fn wait(self: *Fence, timeout_ns: i64) !void {
        if (self.signaled) return;
        var poll_fd = std.posix.pollfd{
            .fd = self.fd,
            .events = std.posix.POLL.IN,
            .revents = 0,
        };
        const timeout_ms: i32 = if (timeout_ns < 0) -1 else @intCast(@divTrunc(timeout_ns, 1000000));
        const result = std.posix.poll(&poll_fd, timeout_ms);
        if (result) |_| {
            if ((poll_fd.revents & std.posix.POLL.IN) != 0) {
                self.signaled = true;
            } else if ((poll_fd.revents & (std.posix.POLL.ERR | std.posix.POLL.HUP)) != 0) {
                self.signaled = true;
            }
        } else |_| {}
        if (!self.signaled and timeout_ns >= 0) return error.Timeout;
    }

    pub fn signal(self: *Fence) !void {
        if (self.signaled) return;
        const val: u64 = 1;
        _ = std.posix.write(self.fd, std.mem.toBytes(val)) catch return error.SignalFailed;
        self.signaled = true;
    }

    pub fn merge(self: *Fence, other: *Fence) !Fence {
        if (self.signaled and other.signaled) {
            return createSignaled();
        }
        if (self.signaled) return .{
            .fd = self.fd,
            .signaled = true,
            .merge_count = self.merge_count + other.merge_count,
        };
        if (other.signaled) return .{
            .fd = other.fd,
            .signaled = true,
            .merge_count = self.merge_count + other.merge_count,
        };
        return .{
            .fd = self.fd,
            .signaled = false,
            .merge_count = self.merge_count + other.merge_count,
        };
    }

    pub fn close(self: *Fence) void {
        if (self.fd >= 0) {
            std.posix.close(self.fd) catch {};
            self.fd = -1;
        }
    }

    pub fn isSignaled(self: *const Fence) bool {
        return self.signaled;
    }

    pub fn getFd(self: *const Fence) std.posix.fd_t {
        return self.fd;
    }

    pub fn reset(self: *Fence) void {
        self.signaled = false;
    }

    pub fn waitForever(self: *Fence) !void {
        return self.wait(-1);
    }

    pub fn waitMs(self: *Fence, timeout_ms: i32) !void {
        return self.wait(@as(i64, timeout_ms) * 1000000);
    }
};

pub const SyncFile = struct {
    fd: std.posix.fd_t,
    name: [64:0]u8,
    status: SyncStatus,

    pub const SyncStatus = enum {
        active,
        signaled,
        error_state,

        pub fn name(self: SyncStatus) []const u8 {
            return switch (self) {
                .active => "Active",
                .signaled => "Signaled",
                .error_state => "Error",
            };
        }
    };

    pub fn create(name: [:0]const u8) !SyncFile {
        const fd = std.posix.memfd_create("waydri-sync", .{ .CLOEXEC = true }) catch return error.CreateFailed;
        var name_buf: [64:0]u8 = std.mem.zeroes([64:0]u8);
        const copy_len = @min(name.len, 63);
        @memcpy(name_buf[0..copy_len], name[0..copy_len]);
        return .{
            .fd = fd,
            .name = name_buf,
            .status = .active,
        };
    }

    pub fn close(self: *SyncFile) void {
        if (self.fd >= 0) {
            std.posix.close(self.fd) catch {};
            self.fd = -1;
        }
        self.status = .error_state;
    }

    pub fn getStatus(self: *const SyncFile) SyncStatus {
        return self.status;
    }

    pub fn isSignaled(self: *const SyncFile) bool {
        return self.status == .signaled;
    }

    pub fn isActive(self: *const SyncFile) bool {
        return self.status == .active;
    }

    pub fn getName(self: *const SyncFile) [:0]const u8 {
        return &self.name;
    }

    pub fn getFd(self: *const SyncFile) std.posix.fd_t {
        return self.fd;
    }
};
