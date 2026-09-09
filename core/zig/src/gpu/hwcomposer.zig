const std = @import("std");

const c = @cImport({
    @cInclude("hardware/hardware.h");
    @cInclude("hardware/hwcomposer.h");
    @cInclude("hardware/hwcomposer2.h");
});

pub const CompositionType = enum(u32) {
    invalid = 0,
    client = 1,
    device = 2,
    solid_color = 3,
    cursor = 4,

    pub fn name(self: CompositionType) []const u8 {
        return switch (self) {
            .invalid => "Invalid",
            .client => "Client",
            .device => "Device",
            .solid_color => "SolidColor",
            .cursor => "Cursor",
        };
    }
};

pub const DisplayCapability = packed struct(u32) {
    secure: bool = false,
    skip_client_color_transform: bool = false,
    hdr_dovi: bool = false,
    hdr10_plus: bool = false,
    hdr10: bool = false,
    dovi: bool = false,
    sdr_white_point: bool = false,
    sideband_stream: bool = false,
    _padding: u24 = 0,
};

pub const Layer = struct {
    layer_id: u32,
    composition_type: CompositionType,
    display_frame: Rect,
    source_crop: Rectf,
    buffer: ?*anyopaque,
    acquire_fence: c_int,
    release_fence: c_int,
    z_order: i32,
    alpha: f32,
    blend_mode: BlendMode,
    transform: Transform,
    visible_region: ?[]Rect,
    damaged_region: ?[]Rect,
    color: Color,
    plane_alpha: u8,
    flags: LayerFlags,

    pub const Rect = struct {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,

        pub fn width(self: Rect) i32 {
            return self.right - self.left;
        }

        pub fn height(self: Rect) i32 {
            return self.bottom - self.top;
        }

        pub fn contains(self: Rect, x: i32, y: i32) bool {
            return x >= self.left and x < self.right and y >= self.top and y < self.bottom;
        }

        pub fn intersects(self: Rect, other: Rect) bool {
            return self.left < other.right and self.right > other.left and
                self.top < other.bottom and self.bottom > other.top;
        }

        pub fn union(self: Rect, other: Rect) Rect {
            return .{
                .left = @min(self.left, other.left),
                .top = @min(self.top, other.top),
                .right = @max(self.right, other.right),
                .bottom = @max(self.bottom, other.bottom),
            };
        }
    };

    pub const Rectf = struct {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,

        pub fn width(self: Rectf) f32 {
            return self.right - self.left;
        }

        pub fn height(self: Rectf) f32 {
            return self.bottom - self.top;
        }
    };

    pub const BlendMode = enum(u32) {
        none = 0,
        premultiplied = 1,
        coverage = 2,
    };

    pub const Transform = packed struct(u32) {
        flip_h: bool = false,
        flip_v: bool = false,
        rotate_90: bool = false,
        rotate_180: bool = false,
        rotate_270: bool = false,
        _padding: u27 = 0,

        pub const NONE = Transform{};
        pub const FLIP_H = Transform{ .flip_h = true };
        pub const FLIP_V = Transform{ .flip_v = true };
        pub const ROT_90 = Transform{ .rotate_90 = true };
        pub const ROT_180 = Transform{ .rotate_180 = true };
        pub const ROT_270 = Transform{ .rotate_270 = true };
    };

    pub const Color = struct {
        r: f32,
        g: f32,
        b: f32,
        a: f32,

        pub const TRANSPARENT = Color{ .r = 0, .g = 0, .b = 0, .a = 0 };
        pub const BLACK = Color{ .r = 0, .g = 0, .b = 0, .a = 1.0 };
        pub const WHITE = Color{ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1.0 };
    };

    pub const LayerFlags = packed struct(u32) {
        skip_color_transform: bool = false,
        sideband_stream: bool = false,
        skip_client_target: bool = false,
        _padding: u29 = 0,
    };

    pub fn createDefault(layer_id: u32) Layer {
        return .{
            .layer_id = layer_id,
            .composition_type = .client,
            .display_frame = .{ .left = 0, .top = 0, .right = 0, .bottom = 0 },
            .source_crop = .{ .left = 0, .top = 0, .right = 0, .bottom = 0 },
            .buffer = null,
            .acquire_fence = -1,
            .release_fence = -1,
            .z_order = 0,
            .alpha = 1.0,
            .blend_mode = .premultiplied,
            .transform = .{},
            .visible_region = null,
            .damaged_region = null,
            .color = .TRANSPARENT,
            .plane_alpha = 255,
            .flags = .{},
        };
    }
};

pub const HwComposer = struct {
    device: ?*anyopaque,
    display_id: u32,
    layers: std.ArrayList(Layer),
    capabilities: DisplayCapability,
    vsync_enabled: bool,
    vsync_period_ns: u64,
    active_config: u32,
    allocator: std.mem.Allocator,

    pub const VsyncCallback = *const fn (u64, ?*anyopaque) void;

    pub fn init(allocator: std.mem.Allocator, display_id: u32) HwComposer {
        return .{
            .device = null,
            .display_id = display_id,
            .layers = std.ArrayList(Layer).init(allocator),
            .capabilities = .{},
            .vsync_enabled = false,
            .vsync_period_ns = 16666666,
            .active_config = 0,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *HwComposer) void {
        self.layers.deinit();
    }

    pub fn setLayerBuffer(self: *HwComposer, layer_index: usize, buffer: *anyopaque, acquire_fence: c_int) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].buffer = buffer;
        self.layers.items[layer_index].acquire_fence = acquire_fence;
    }

    pub fn setLayerDisplayFrame(self: *HwComposer, layer_index: usize, frame: Layer.Rect) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].display_frame = frame;
    }

    pub fn setLayerSourceCrop(self: *HwComposer, layer_index: usize, crop: Layer.Rectf) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].source_crop = crop;
    }

    pub fn setLayerCompositionType(self: *HwComposer, layer_index: usize, comp_type: CompositionType) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].composition_type = comp_type;
    }

    pub fn setLayerBlendMode(self: *HwComposer, layer_index: usize, blend: Layer.BlendMode) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].blend_mode = blend;
    }

    pub fn setLayerAlpha(self: *HwComposer, layer_index: usize, alpha: f32) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].alpha = alpha;
    }

    pub fn setLayerZOrder(self: *HwComposer, layer_index: usize, z: i32) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].z_order = z;
    }

    pub fn setLayerTransform(self: *HwComposer, layer_index: usize, transform: Layer.Transform) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].transform = transform;
    }

    pub fn setLayerColor(self: *HwComposer, layer_index: usize, color: Layer.Color) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        self.layers.items[layer_index].color = color;
    }

    pub fn addLayer(self: *HwComposer) !usize {
        const idx = self.layers.items.len;
        try self.layers.append(.createDefault(@intCast(idx)));
        return idx;
    }

    pub fn removeLayer(self: *HwComposer, layer_index: usize) !void {
        if (layer_index >= self.layers.items.len) return error.InvalidLayerIndex;
        _ = self.layers.swapRemove(layer_index);
    }

    pub fn presentDisplay(self: *HwComposer) !PresentResult {
        var release_fences: std.BoundedArray(c_int, 32) = .{};
        for (self.layers.items) |layer| {
            if (layer.buffer != null) {
                release_fences.append(layer.release_fence) catch break;
            }
        }
        return .{
            .present_fence = -1,
            .release_fences = release_fences.slice(),
            .display_refresh_rate: 1000000000.0 / @as(f64, @floatFromInt(self.vsync_period_ns)),
        };
    }

    pub fn getReleaseFences(self: *const HwComposer) []const c_int {
        var fences: std.BoundedArray(c_int, 32) = .{};
        for (self.layers.items) |layer| {
            if (layer.release_fence >= 0) {
                fences.append(layer.release_fence) catch break;
            }
        }
        return fences.slice();
    }

    pub fn enableVsync(self: *HwComposer, enable: bool) void {
        self.vsync_enabled = enable;
    }

    pub fn getLayerCount(self: *const HwComposer) usize {
        return self.layers.items.len;
    }

    pub fn getLayer(self: *const HwComposer, index: usize) ?Layer {
        if (index >= self.layers.items.len) return null;
        return self.layers.items[index];
    }

    pub fn setVsyncPeriod(self: *HwComposer, period_ns: u64) void {
        self.vsync_period_ns = period_ns;
    }

    pub const PresentResult = struct {
        present_fence: c_int,
        release_fences: []const c_int,
        display_refresh_rate: f64,
    };
};
