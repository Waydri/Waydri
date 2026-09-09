const std = @import("std");

pub const Semaphore = struct {
    count: u32,
    max_count: u32,
    mutex: std.Thread.Mutex,
    cond: std.Thread.Condition,

    pub fn init(initial: u32, max_count: u32) Semaphore {
        return .{
            .count = initial,
            .max_count = max_count,
            .mutex = .{},
            .cond = .{},
        };
    }

    pub fn destroy(self: *Semaphore) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.count = 0;
        self.max_count = 0;
    }

    pub fn post(self: *Semaphore) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        if (self.count < self.max_count) {
            self.count += 1;
            self.cond.signal();
        }
    }

    pub fn tryWait(self: *Semaphore) bool {
        self.mutex.lock();
        defer self.mutex.unlock();
        if (self.count > 0) {
            self.count -= 1;
            return true;
        }
        return false;
    }

    pub fn wait(self: *Semaphore) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        while (self.count == 0) {
            self.cond.wait(&self.mutex);
        }
        self.count -= 1;
    }

    pub fn timedWait(self: *Semaphore, timeout_ns: u64) bool {
        self.mutex.lock();
        defer self.mutex.unlock();
        if (self.count > 0) {
            self.count -= 1;
            return true;
        }
        const deadline = std.time.nanoTimestamp() + @as(i128, timeout_ns);
        while (self.count == 0) {
            const now = std.time.nanoTimestamp();
            if (now >= deadline) return false;
            const remaining: u64 = @intCast(@max(0, deadline - now));
            self.cond.timedWait(&self.mutex, remaining) catch return false;
        }
        self.count -= 1;
        return true;
    }

    pub fn getCount(self: *Semaphore) u32 {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.count;
    }

    pub fn getMaxCount(self: *const Semaphore) u32 {
        return self.max_count;
    }

    pub fn reset(self: *Semaphore) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.count = 0;
    }

    pub fn setValue(self: *Semaphore, value: u32) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.count = @min(value, self.max_count);
        if (self.count > 0) {
            self.cond.signal();
        }
    }

    pub fn waitAll(self: *Semaphore, count: u32) void {
        var acquired: u32 = 0;
        while (acquired < count) {
            self.wait();
            acquired += 1;
        }
    }

    pub fn tryWaitAll(self: *Semaphore, count: u32) bool {
        self.mutex.lock();
        if (self.count >= count) {
            self.count -= count;
            self.mutex.unlock();
            return true;
        }
        self.mutex.unlock();
        return false;
    }

    pub fn releaseAll(self: *Semaphore) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.count = self.max_count;
        self.cond.broadcast();
    }
};
