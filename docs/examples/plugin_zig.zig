const std = @import("std");
const Allocator = std.mem.Allocator;

pub const Plugin = struct {
    name: []const u8,
    version: []const u8,
    elapsed: f64,
    update_interval: f64,

    pub fn init(allocator: Allocator) Plugin {
        _ = allocator;
        return Plugin{
            .name = "zig-counter",
            .version = "1.0.0",
            .elapsed = 0.0,
            .update_interval = 2.0,
        };
    }

    pub fn onStart(self: *Plugin, ctx: *PluginContext) !void {
        const window_count = ctx.windowCount();
        self.label = try std.fmt.allocPrint(
            ctx.allocator,
            "Zig Plugin: {d} windows",
            .{window_count},
        );
    }

    pub fn onStop(self: *Plugin) void {
        self.elapsed = 0.0;
    }

    pub fn onTick(self: *Plugin, dt: f64, ctx: *PluginContext) void {
        self.elapsed += dt;
        if (self.elapsed >= self.update_interval) {
            self.elapsed = 0.0;
            const count = ctx.windowCount();
            ctx.log("Tick: {d} windows on screen", .{count});
        }
    }
};

pub const PluginContext = struct {
    allocator: Allocator,
    window_count_fn: *const fn () usize,
    log_fn: *const fn ([]const u8) void,

    pub fn windowCount(self: PluginContext) usize {
        return self.window_count_fn();
    }

    pub fn log(self: PluginContext, comptime fmt: []const u8, args: anytype) void {
        const msg = std.fmt.allocPrint(self.allocator, fmt, args) catch return;
        self.log_fn(msg);
        self.allocator.destroy(msg);
    }
};

pub const Capability = enum {
    compositor,
    network,
    fs_read,
    fs_write,
    input,
    rpc,
    animation,
};

export fn create_plugin() ?*Plugin {
    const allocator = std.heap.page_allocator;
    const plugin = allocator.create(Plugin) catch return null;
    plugin.* = Plugin.init(allocator);
    return plugin;
}

export fn destroy_plugin(plugin: ?*Plugin) void {
    if (plugin) |p| {
        std.heap.page_allocator.destroy(p);
    }
}
