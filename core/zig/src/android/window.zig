const std = @import("std");

const c = @cImport({
    @cInclude("android/native_window.h");
    @cInclude("android/window.h");
});

pub const WindowManager = struct {
    windows: std.ArrayList(WindowEntry),
    allocator: std.mem.Allocator,
    next_id: u32,

    pub const WindowEntry = struct {
        id: u32,
        title: [:0]const u8,
        icon_path: ?[:0]const u8,
        width: u32,
        height: u32,
        fullscreen: bool,
        visible: bool,
        surface: ?*anyopaque,
        properties: WindowProperties,
    };

    pub const WindowProperties = struct {
        transparent: bool = false,
        focusable: bool = true,
        resizable: bool = true,
        decorated: bool = true,
        max_width: ?u32 = null,
        max_height: ?u32 = null,
        min_width: ?u32 = null,
        min_height: ?u32 = null,
    };

    pub const WindowCreateConfig = struct {
        title: [:0]const u8,
        width: u32,
        height: u32,
        properties: WindowProperties = .{},
    };

    pub fn init(allocator: std.mem.Allocator) WindowManager {
        return .{
            .windows = std.ArrayList(WindowEntry).init(allocator),
            .allocator = allocator,
            .next_id = 1,
        };
    }

    pub fn deinit(self: *WindowManager) void {
        for (self.windows.items) |entry| {
            self.allocator.free(entry.title);
            if (entry.icon_path) |path| {
                self.allocator.free(path);
            }
        }
        self.windows.deinit();
    }

    pub fn createWindow(self: *WindowManager, config: WindowCreateConfig) !u32 {
        const id = self.next_id;
        self.next_id += 1;

        const title_dup = try self.allocator.dupeZ(u8, config.title);

        try self.windows.append(.{
            .id = id,
            .title = title_dup,
            .icon_path = null,
            .width = config.width,
            .height = config.height,
            .fullscreen = false,
            .visible = true,
            .surface = null,
            .properties = config.properties,
        });

        return id;
    }

    pub fn destroyWindow(self: *WindowManager, id: u32) !void {
        for (self.windows.items, 0..) |entry, i| {
            if (entry.id == id) {
                self.allocator.free(entry.title);
                if (entry.icon_path) |path| {
                    self.allocator.free(path);
                }
                _ = self.windows.swapRemove(i);
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn setTitle(self: *WindowManager, id: u32, title: [:0]const u8) !void {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                self.allocator.free(entry.title);
                entry.title = try self.allocator.dupeZ(u8, title);
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn setIcon(self: *WindowManager, id: u32, icon_path: [:0]const u8) !void {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                if (entry.icon_path) |old_path| {
                    self.allocator.free(old_path);
                }
                entry.icon_path = try self.allocator.dupeZ(u8, icon_path);
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn toggleFullscreen(self: *WindowManager, id: u32) !bool {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                entry.fullscreen = !entry.fullscreen;
                return entry.fullscreen;
            }
        }
        return error.WindowNotFound;
    }

    pub fn setFullscreen(self: *WindowManager, id: u32, fullscreen: bool) !void {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                entry.fullscreen = fullscreen;
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn setVisible(self: *WindowManager, id: u32, visible: bool) !void {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                entry.visible = visible;
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn setSurface(self: *WindowManager, id: u32, surface: ?*anyopaque) !void {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                entry.surface = surface;
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn setDimensions(self: *WindowManager, id: u32, width: u32, height: u32) !void {
        for (self.windows.items) |*entry| {
            if (entry.id == id) {
                const props = entry.properties;
                entry.width = if (props.max_width) |max| @min(width, max) else width;
                entry.height = if (props.max_height) |max| @min(height, max) else height;
                if (props.min_width) |min_val| entry.width = @max(entry.width, min_val);
                if (props.min_height) |min_val| entry.height = @max(entry.height, min_val);
                return;
            }
        }
        return error.WindowNotFound;
    }

    pub fn getWindow(self: *const WindowManager, id: u32) ?WindowEntry {
        for (self.windows.items) |entry| {
            if (entry.id == id) return entry;
        }
        return null;
    }

    pub fn findWindowByTitle(self: *const WindowManager, title: [:0]const u8) ?WindowEntry {
        for (self.windows.items) |entry| {
            if (std.mem.eql(u8, entry.title, title)) return entry;
        }
        return null;
    }

    pub fn getWindowCount(self: *const WindowManager) usize {
        return self.windows.items.len;
    }

    pub fn getAllWindows(self: *const WindowManager) []const WindowEntry {
        return self.windows.items;
    }

    pub fn getVisibleWindows(self: *const WindowManager) []const WindowEntry {
        var visible: std.BoundedArray(WindowEntry, 64) = .{};
        for (self.windows.items) |entry| {
            if (entry.visible) {
                visible.append(entry) catch break;
            }
        }
        return visible.slice();
    }

    pub fn bringToFront(self: *WindowManager, id: u32) !void {
        for (self.windows.items, 0..) |entry, i| {
            if (entry.id == id) {
                const window = self.windows.swapRemove(i);
                try self.windows.append(window);
                return;
            }
        }
        return error.WindowNotFound;
    }
};
