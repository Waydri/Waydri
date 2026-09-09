const std = @import("std");

const c = @cImport({
    @cInclude("xf86drm.h");
    @cInclude("libdrm/drm.h");
    @cInclude("libdrm/drm_mode.h");
});

pub const ConnectorType = enum(u32) {
    unknown = c.DRM_MODE_CONNECTOR_Unknown,
    vga = c.DRM_MODE_CONNECTOR_VGA,
    dvii = c.DRM_MODE_CONNECTOR_DVII,
    dvid = c.DRM_MODE_CONNECTOR_DVID,
    dvia = c.DRM_MODE_CONNECTOR_DVIA,
    composite = c.DRM_MODE_CONNECTOR_Composite,
    svideo = c.DRM_MODE_CONNECTOR_SVIDEO,
    lvds = c.DRM_MODE_CONNECTOR_LVDS,
    component = c.DRM_MODE_CONNECTOR_Component,
    nine_pin_din = c.DRM_MODE_CONNECTOR_9PinDIN,
    displayport = c.DRM_MODE_CONNECTOR_DisplayPort,
    hdmi = c.DRM_MODE_CONNECTOR_HDMIA,
    hdmi_b = c.DRM_MODE_CONNECTOR_HDMIB,
    tv = c.DRM_MODE_CONNECTOR_TV,
    edp = c.DRM_MODE_CONNECTOR_eDP,
    virt = c.DRM_MODE_CONNECTOR_VIRTUAL,
    dsi = c.DRM_MODE_CONNECTOR_DSI,
    dpi = c.DRM_MODE_CONNECTOR_DPI,

    pub fn name(self: ConnectorType) []const u8 {
        return switch (self) {
            .vga => "VGA",
            .hdmi => "HDMI",
            .displayport => "DisplayPort",
            .edp => "eDP",
            .dsi => "DSI",
            .lvds => "LVDS",
            .tv => "TV",
            .dpi => "DPI",
            .unknown => "Unknown",
            else => "Other",
        };
    }
};

pub const ConnectorStatus = enum(c_int) {
    connected = c.DRM_MODE_CONNECTED,
    disconnected = c.DRM_MODE_DISCONNECTED,
    unknown = c.DRM_MODE_UNKNOWNCONNECTION,

    pub fn isConnected(self: ConnectorStatus) bool {
        return self == .connected;
    }
};

pub const Connector = struct {
    connector_id: u32,
    connector_type: ConnectorType,
    connector_type_id: u32,
    status: ConnectorStatus,
    edid: ?[]const u8,
    modes: []DisplayMode,
    encoder_id: u32,
    mm_width: u32,
    mm_height: u32,
    subpixel: c.drm_subpixel,

    pub const DisplayMode = struct {
        mode_id: u32,
        clock_khz: u32,
        hdisplay: u16,
        hsync_start: u16,
        hsync_end: u16,
        htotal: u16,
        vdisplay: u16,
        vsync_start: u16,
        vsync_end: u16,
        vtotal: u16,
        vrefresh: u32,
        flags: u32,
        type: u32,
        name: [DRM_DISPLAY_MODE_LEN:0]u8,

        pub const DRM_DISPLAY_MODE_LEN = 32;

        pub fn fromDrmMode(mode: c.struct_drm_mode_modeinfo) DisplayMode {
            var name_buf: [DRM_DISPLAY_MODE_LEN:0]u8 = std.mem.zeroes([DRM_DISPLAY_MODE_LEN:0]u8);
            const src_name: [*]const u8 = @ptrCast(&mode.name);
            const len = std.mem.indexOfScalar(u8, src_name[0..DRM_DISPLAY_MODE_LEN], 0) orelse DRM_DISPLAY_MODE_LEN;
            @memcpy(name_buf[0..len], src_name[0..len]);
            return .{
                .mode_id = 0,
                .clock_khz = mode.clock,
                .hdisplay = mode.hdisplay,
                .hsync_start = mode.hsync_start,
                .hsync_end = mode.hsync_end,
                .htotal = mode.htotal,
                .vdisplay = mode.vdisplay,
                .vsync_start = mode.vsync_start,
                .vsync_end = mode.vsync_end,
                .vtotal = mode.vtotal,
                .vrefresh = mode.vrefresh,
                .flags = mode.flags,
                .type = @intCast(mode.type),
                .name = name_buf,
            };
        }

        pub fn isPreferred(self: DisplayMode) bool {
            return (self.type & c.DRM_MODE_TYPE_PREFERRED) != 0;
        }

        pub fn isDetailed(self: DisplayMode) bool {
            return (self.type & c.DRM_MODE_TYPE_DRIVER) != 0;
        }

        pub fn width(self: DisplayMode) u32 {
            return self.hdisplay;
        }

        pub fn height(self: DisplayMode) u32 {
            return self.vdisplay;
        }

        pub fn refreshMhz(self: DisplayMode) u32 {
            return self.vrefresh;
        }

        pub fn dotClock(self: DisplayMode) u64 {
            return @as(u64, self.clock_khz) * 1000;
        }
    };
};

pub const Plane = struct {
    plane_id: u32,
    crtc_id: u32,
    crtc_index: u32,
    fb_id: u32,
    x: u32,
    y: u32,
    gamma_size: u32,
    possible_crtcs: u32,
    possible_encoders: u32,
    gamma_cursor_size: u32,
    plane_type: PlaneType,

    pub const PlaneType = enum(u32) {
        overlay = 1,
        primary = 2,
        cursor = 4,

        pub fn name(self: PlaneType) []const u8 {
            return switch (self) {
                .overlay => "Overlay",
                .primary => "Primary",
                .cursor => "Cursor",
            };
        }
    };

    pub fn isPrimary(self: Plane) bool {
        return self.plane_type == .primary;
    }

    pub fn isOverlay(self: Plane) bool {
        return self.plane_type == .overlay;
    }

    pub fn isCursor(self: Plane) bool {
        return self.plane_type == .cursor;
    }
};

pub const Crtc = struct {
    crtc_id: u32,
    buffer_id: u32,
    x: u32,
    y: u32,
    gamma_size: u32,
    mode_valid: bool,
    mode: ?Connector.DisplayMode,

    pub fn isActive(self: Crtc) bool {
        return self.buffer_id != 0 and self.mode_valid;
    }

    pub fn getResolution(self: Crtc) ?struct { width: u32, height: u32 } {
        if (self.mode) |m| {
            return .{ .width = m.width(), .height = m.height() };
        }
        return null;
    }
};

pub const DumbBuffer = struct {
    handle: u32,
    pitch: u32,
    size: u64,
    width: u32,
    height: u32,
    bpp: u32,
    depth: u32,
    vaddr: ?[*]u8,
};

pub const EdidInfo = struct {
    manufacturer_id: [4]u8,
    product_code: u16,
    serial_number: u32,
    manufacture_week: u8,
    manufacture_year: u16,
    edid_version: u8,
    edid_revision: u8,
    max_width_cm: u8,
    max_height_cm: u8,
    gamma: u16,
    features: u32,
    chroma: ChromaInfo,
    timings: TimingInfo,
    monitor_name: [14]u8,
    monitor_ranges: MonitorRanges,

    pub const ChromaInfo = struct {
        red_x: u16,
        red_y: u16,
        green_x: u16,
        green_y: u16,
        blue_x: u16,
        blue_y: u16,
        white_x: u16,
        white_y: u16,
    };

    pub const TimingInfo = struct {
        horiz_active: u16,
        horiz_blank: u16,
        vert_active: u16,
        vert_blank: u16,
        pixel_clock_khz: u16,
        horiz_sync_offset: u8,
        horiz_sync_width: u8,
        vert_sync_offset_low: u8,
        vert_sync_width_low: u8,
    };

    pub const MonitorRanges = struct {
        min_vfreq: u8,
        max_vfreq: u8,
        min_hfreq_khz: u16,
        max_hfreq_khz: u16,
        pixel_clock_max_mhz: u16,
    };
};

const drm_get_cap = extern struct {
    capability: u64,
    value: u64,
};

pub const DrmDevice = struct {
    fd: std.posix.fd_t,
    resources: ?DrmResources,
    planes: ?[]Plane,
    connected: bool,
    has_atomic: bool,
    has_dumb: bool,

    pub const DrmResources = struct {
        fbs: []u32,
        crtcs: []Crtc,
        connectors: []Connector,
        encoders: []Encoder,
    };

    pub const Encoder = struct {
        encoder_id: u32,
        encoder_type: u32,
        crtc_id: u32,
        possible_crtcs: u32,
        possible_clones: u32,
    };

    pub fn open(device_path: [:0]const u8) !DrmDevice {
        const path_ptr: [*:0]const u8 = device_path.ptr;
        const fd = std.posix.open(path_ptr, .{ .ACCMODE = .RDWR, .CLOEXEC = true }, 0) catch return error.OpenFailed;
        return .{
            .fd = fd,
            .resources = null,
            .planes = null,
            .connected = false,
            .has_atomic = false,
            .has_dumb = false,
        };
    }

    pub fn close(self: *DrmDevice) void {
        if (self.planes) |planes| {
            std.heap.page_allocator.free(planes);
        }
        if (self.resources) |*res| {
            std.heap.page_allocator.free(res.fbs);
            std.heap.page_allocator.free(res.crtcs);
            std.heap.page_allocator.free(res.connectors);
            std.heap.page_allocator.free(res.encoders);
        }
        std.posix.close(self.fd);
    }

    pub fn getResources(self: *DrmDevice) !DrmResources {
        var res_req = std.mem.zeroes(c.struct_drm_mode_card_res);
        res_req.count_fbs = 0;
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETRESOURCES, @intCast(@intFromPtr(&res_req)));

        const fb_ids = try std.heap.page_allocator.alloc(u32, res_req.count_fbs);
        var i: u32 = 0;
        while (i < res_req.count_fbs) : (i += 1) {
            fb_ids[i] = 0;
        }
        res_req.fb_id_ptr = @intFromPtr(fb_ids.ptr);

        const crtc_ids = try std.heap.page_allocator.alloc(u32, res_req.count_crtcs);
        i = 0;
        while (i < res_req.count_crtcs) : (i += 1) {
            crtc_ids[i] = 0;
        }
        res_req.crtc_id_ptr = @intFromPtr(crtc_ids.ptr);

        const conn_ids = try std.heap.page_allocator.alloc(u32, res_req.count_connectors);
        i = 0;
        while (i < res_req.count_connectors) : (i += 1) {
            conn_ids[i] = 0;
        }
        res_req.connector_id_ptr = @intFromPtr(conn_ids.ptr);

        const enc_ids = try std.heap.page_allocator.alloc(u32, res_req.count_encoders);
        i = 0;
        while (i < res_req.count_encoders) : (i += 1) {
            enc_ids[i] = 0;
        }
        res_req.encoder_id_ptr = @intFromPtr(enc_ids.ptr);

        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETRESOURCES, @intCast(@intFromPtr(&res_req)));

        var crtcs = try std.heap.page_allocator.alloc(Crtc, res_req.count_crtcs);
        i = 0;
        while (i < res_req.count_crtcs) : (i += 1) {
            var crtc_req = std.mem.zeroes(c.struct_drm_mode_crtc);
            crtc_req.crtc_id = crtc_ids[i];
            _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETCRTC, @intCast(@intFromPtr(&crtc_req)));
            var mode_info: ?Connector.DisplayMode = null;
            if (crtc_req.mode_valid != 0) {
                mode_info = Connector.DisplayMode.fromDrmMode(crtc_req.mode);
            }
            crtcs[i] = .{
                .crtc_id = crtc_req.crtc_id,
                .buffer_id = crtc_req.fb_id,
                .x = crtc_req.x,
                .y = crtc_req.y,
                .gamma_size = crtc_req.gamma_size,
                .mode_valid = crtc_req.mode_valid != 0,
                .mode = mode_info,
            };
        }

        var connectors = try std.heap.page_allocator.alloc(Connector, res_req.count_connectors);
        i = 0;
        while (i < res_req.count_connectors) : (i += 1) {
            var conn_req = std.mem.zeroes(c.struct_drm_mode_get_connector);
            conn_req.connector_id = conn_ids[i];
            _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETCONNECTOR, @intCast(@intFromPtr(&conn_req)));
            connectors[i] = .{
                .connector_id = conn_req.connector_id,
                .connector_type = @enumFromInt(conn_req.connector_type),
                .connector_type_id = conn_req.connector_type_id,
                .status = @enumFromInt(conn_req.connection),
                .edid = null,
                .modes = &.{},
                .encoder_id = if (conn_req.encoder_id != 0) conn_req.encoder_id else 0,
                .mm_width = conn_req.mm_width,
                .mm_height = conn_req.mm_height,
                .subpixel = conn_req.subpixel,
            };
        }

        var encoders = try std.heap.page_allocator.alloc(Encoder, res_req.count_encoders);
        i = 0;
        while (i < res_req.count_encoders) : (i += 1) {
            var enc_req = std.mem.zeroes(c.struct_drm_mode_get_encoder);
            enc_req.encoder_id = enc_ids[i];
            _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETENCODER, @intCast(@intFromPtr(&enc_req)));
            encoders[i] = .{
                .encoder_id = enc_req.encoder_id,
                .encoder_type = enc_req.encoder_type,
                .crtc_id = enc_req.crtc_id,
                .possible_crtcs = enc_req.possible_crtcs,
                .possible_clones = enc_req.possible_clones,
            };
        }

        return .{
            .fbs = fb_ids,
            .crtcs = crtcs,
            .connectors = connectors,
            .encoders = encoders,
        };
    }

    pub fn getPlaneResources(self: *DrmDevice) ![]Plane {
        var plane_req = std.mem.zeroes(c.struct_drm_mode_get_plane_res);
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETPLANERESOURCES, @intCast(@intFromPtr(&plane_req)));

        const plane_ids = try std.heap.page_allocator.alloc(u32, plane_req.count_planes);
        var i: u32 = 0;
        while (i < plane_req.count_planes) : (i += 1) {
            plane_ids[i] = 0;
        }
        plane_req.plane_id_ptr = @intFromPtr(plane_ids.ptr);
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETPLANERESOURCES, @intCast(@intFromPtr(&plane_req)));

        var planes = try std.heap.page_allocator.alloc(Plane, plane_req.count_planes);
        i = 0;
        while (i < plane_req.count_planes) : (i += 1) {
            var plane_info = std.mem.zeroes(c.struct_drm_mode_get_plane);
            plane_info.plane_id = plane_ids[i];
            _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_GETPLANE, @intCast(@intFromPtr(&plane_info)));
            planes[i] = .{
                .plane_id = plane_info.plane_id,
                .crtc_id = plane_info.crtc_id,
                .crtc_index = plane_info.crtc_id,
                .fb_id = plane_info.fb_id,
                .x = plane_info.x,
                .y = plane_info.y,
                .gamma_size = plane_info.gamma_size,
                .possible_crtcs = plane_info.possible_crtcs,
                .possible_encoders = plane_info.possible_encoders,
                .gamma_cursor_size = 0,
                .plane_type = @enumFromInt(plane_info.plane_type),
            };
        }

        self.planes = planes;
        return planes;
    }

    pub fn createDumbBuffer(self: *DrmDevice, width: u32, height: u32, bpp: u32, depth: u32) !DumbBuffer {
        var create_req = std.mem.zeroes(c.struct_drm_mode_create_dumb);
        create_req.width = width;
        create_req.height = height;
        create_req.bpp = bpp;
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_CREATE_DUMB, @intCast(@intFromPtr(&create_req)));
        if (create_req.handle == 0) return error.CreateDumbFailed;

        return .{
            .handle = create_req.handle,
            .pitch = create_req.pitch,
            .size = create_req.size,
            .width = width,
            .height = height,
            .bpp = bpp,
            .depth = depth,
            .vaddr = null,
        };
    }

    pub fn destroyDumbBuffer(self: *DrmDevice, buffer: *DumbBuffer) !void {
        var destroy_req = std.mem.zeroes(c.struct_drm_mode_destroy_dumb);
        destroy_req.handle = buffer.handle;
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_DESTROY_DUMB, @intCast(@intFromPtr(&destroy_req)));
        buffer.handle = 0;
    }

    pub fn modeSetCrtc(self: *DrmDevice, crtc_id: u32, fb_id: u32, x: u32, y: u32, connectors: []const u32, mode: ?Connector.DisplayMode) !void {
        var set_crtc = std.mem.zeroes(c.struct_drm_mode_crtc);
        set_crtc.crtc_id = crtc_id;
        set_crtc.fb_id = fb_id;
        set_crtc.x = x;
        set_crtc.y = y;
        if (connectors.len > 0) {
            set_crtc.set_connectors_ptr = @intFromPtr(connectors.ptr);
            set_crtc.count_connectors = @intCast(connectors.len);
        }
        if (mode) |m| {
            set_crtc.mode_valid = 1;
            set_crtc.mode.clock = m.clock_khz;
            set_crtc.mode.hdisplay = m.hdisplay;
            set_crtc.mode.hsync_start = m.hsync_start;
            set_crtc.mode.hsync_end = m.hsync_end;
            set_crtc.mode.htotal = m.htotal;
            set_crtc.mode.vdisplay = m.vdisplay;
            set_crtc.mode.vsync_start = m.vsync_start;
            set_crtc.mode.vsync_end = m.vsync_end;
            set_crtc.mode.vtotal = m.vtotal;
            set_crtc.mode.vrefresh = m.vrefresh;
            set_crtc.mode.flags = m.flags;
        }
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_MODE_SETCRTC, @intCast(@intFromPtr(&set_crtc)));
    }

    pub fn parseEdid(data: []const u8) ?EdidInfo {
        if (data.len < 128) return null;
        if (data[0] != 0x00 or data[1] != 0xFF or data[2] != 0xFF or data[3] != 0xFF or
            data[4] != 0xFF or data[5] != 0xFF or data[6] != 0xFF or data[7] != 0x00)
            return null;

        var info = std.mem.zeroes(EdidInfo);
        info.edid_version = data[18];
        info.edid_revision = data[19];
        info.max_width_cm = data[21];
        info.max_height_cm = data[22];
        info.gamma = (@as(u16, data[23]) + 100) * 10;

        info.manufacturer_id[0] = @intCast((@as(u16, data[8]) << 2) | (@as(u16, data[9]) >> 6));
        info.manufacturer_id[1] = @intCast(((@as(u16, data[9]) & 0x3F) << 4) | (@as(u16, data[10]) >> 4));
        info.manufacturer_id[2] = @intCast(((@as(u16, data[10]) & 0x0F) << 6) | (@as(u16, data[11]) >> 2));
        info.manufacturer_id[3] = @intCast(((@as(u16, data[11]) & 0x03) << 8) | @as(u16, data[12]));

        info.product_code = @as(u16, data[15]) << 8 | data[14];
        info.serial_number = @as(u32, data[12]) << 24 | @as(u32, data[13]) << 16 | @as(u32, data[14]) << 8 | data[15];
        info.manufacture_week = data[16];
        info.manufacture_year = @as(u16, data[17]) + 1990;

        info.chroma.red_x = @as(u16, data[25]) << 2 | (@as(u16, data[26]) >> 6);
        info.chroma.red_y = @as(u16, (data[26] >> 2) & 0x0F);
        info.chroma.green_x = @as(u16, (data[26] & 0x03) << 2) | (@as(u16, data[27]) >> 6);
        info.chroma.green_y = @as(u16, (data[27] >> 2) & 0x0F);
        info.chroma.blue_x = @as(u16, (data[27] & 0x03) << 2) | (@as(u16, data[28]) >> 6);
        info.chroma.blue_y = @as(u16, (data[28] >> 2) & 0x0F);
        info.chroma.white_x = @as(u16, (data[28] & 0x03) << 2) | (@as(u16, data[29]) >> 6);
        info.chroma.white_y = @as(u16, (data[29] >> 2) & 0x0F);

        var block_offset: usize = 54;
        var name_idx: usize = 0;
        while (block_offset + 18 <= data.len and name_idx < 13) : (block_offset += 18) {
            if (data[block_offset] == 0 and data[block_offset + 1] == 0 and
                data[block_offset + 2] == 0 and data[block_offset + 3] == 0xFC)
            {
                var j: usize = 5;
                while (j < 18 and data[block_offset + j] != 0x0A) : (j += 1) {
                    if (name_idx < 13) {
                        info.monitor_name[name_idx] = data[block_offset + j];
                        name_idx += 1;
                    }
                }
                info.monitor_name[name_idx] = 0;
            }
            if (data[block_offset] == 0 and data[block_offset + 1] == 0 and
                data[block_offset + 2] == 0 and data[block_offset + 3] == 0xFD)
            {
                info.monitor_ranges.min_vfreq = data[block_offset + 5];
                info.monitor_ranges.max_vfreq = data[block_offset + 6];
                info.monitor_ranges.min_hfreq_khz = @as(u16, data[block_offset + 7]) * 1000;
                info.monitor_ranges.max_hfreq_khz = @as(u16, data[block_offset + 8]) * 1000;
                info.monitor_ranges.pixel_clock_max_mhz = @as(u16, data[block_offset + 9]) * 10;
            }
        }

        return info;
    }

    pub fn getFd(self: *const DrmDevice) std.posix.fd_t {
        return self.fd;
    }

    pub fn checkCapabilities(self: *DrmDevice) !void {
        var cap: u64 = 0;
        const args = drm_get_cap{ .capability = c.DRM_CAP_ATOMIC, .value = 0 };
        _ = std.io.linux.ioctl(self.fd, c.DRM_IOCTL_GET_CAP, @intCast(@intFromPtr(&args)));
        cap = args.value;
        self.has_atomic = cap != 0;
        self.has_dumb = true;
    }

    pub fn supportsAtomic(self: *const DrmDevice) bool {
        return self.has_atomic;
    }
};
