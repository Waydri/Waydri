const std = @import("std");

pub const MaliArchitecture = enum {
    utgard,
    midgard,
    bifrost,
    valhall,

    pub fn name(self: MaliArchitecture) []const u8 {
        return switch (self) {
            .utgard => "Utgard",
            .midgard => "Midgard",
            .bifrost => "Bifrost",
            .valhall => "Valhall",
        };
    }

    pub fn generation(self: MaliArchitecture) u32 {
        return switch (self) {
            .utgard => 1,
            .midgard => 2,
            .bifrost => 3,
            .valhall => 4,
        };
    }
};

pub const MaliFeatures = packed struct(u32) {
    has_bifrost: bool = false,
    has_midgard: bool = false,
    has_valhall: bool = false,
    has_utgard: bool = false,
    has_afbc: bool = false,
    has_yuv: bool = false,
    has_early_z: bool = false,
    has_thread_group_fix: bool = false,
    has_tiler: bool = false,
    has_job_slot_affinity: bool = false,
    hasprot: bool = false,
    has_fault: bool = false,
    _padding: u20 = 0,
};

pub const MaliProduct = enum(u32) {
    t600 = 0x600,
    t620 = 0x620,
    t720 = 0x720,
    t760 = 0x760,
    t820 = 0x820,
    t860 = 0x860,
    t880 = 0x880,
    g31 = 0x07000061,
    g51 = 0x07000051,
    g52 = 0x07000052,
    g57 = 0x09000057,
    g71 = 0x06000051,
    g72 = 0x06000052,
    g76 = 0x06000056,
    g77 = 0x09000057,
    g78 = 0x09000058,
    g710 = 0x09000071,
    g715 = 0x09000075,
    g720 = 0x09000072,

    pub fn architecture(self: MaliProduct) MaliArchitecture {
        return switch (self) {
            .t600, .t620, .t720, .t760, .t820, .t860, .t880 => .midgard,
            .g31, .g51, .g52 => .bifrost,
            .g71, .g72, .g76 => .bifrost,
            .g77, .g78, .g710, .g715, .g720 => .valhall,
        };
    }
};

pub const MaliDriver = struct {
    product_id: u32,
    product: MaliProduct,
    features: MaliFeatures,
    architecture: MaliArchitecture,
    num_cores: u32,
    hw_issues: std.ArrayList(u32),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) MaliDriver {
        return .{
            .product_id = 0,
            .product = .g31,
            .features = .{},
            .architecture = .bifrost,
            .num_cores = 1,
            .hw_issues = std.ArrayList(u32).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *MaliDriver) void {
        self.hw_issues.deinit();
    }

    pub fn detect(self: *MaliDriver) !void {
        self.product_id = try self.readProductId();
        self.product = @enumFromInt(self.product_id);
        self.architecture = self.product.architecture();
        self.num_cores = try self.readCoreCount();
        self.features.has_bifrost = self.architecture == .bifrost;
        self.features.has_midgard = self.architecture == .midgard;
        self.features.has_valhall = self.architecture == .valhall;
        self.features.has_utgard = self.architecture == .utgard;
        self.features.has_afbc = self.architecture.generation() >= 3;
        self.features.has_yuv = self.architecture.generation() >= 3;
        self.features.has_early_z = self.architecture.generation() >= 2;
        self.features.has_thread_group_fix = self.architecture == .valhall;
        self.features.has_tiler = self.architecture.generation() >= 2;
        self.features.has_job_slot_affinity = self.architecture.generation() >= 3;
        self.features.hasprot = self.architecture.generation() >= 2;
        self.features.has_fault = self.architecture.generation() >= 2;
        self.detectHardwareIssues();
    }

    fn readProductId(self: *MaliDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/devices/platform/soc/*/gpu/product_id",
            "/sys/devices/platform/*/mali.*/product_id",
            "/sys/class/misc/mali0/device/product_id",
        };
        for (paths) |path| {
            if (std.fs.cwd().readFileAlloc(path, std.heap.page_allocator, 4096)) |content| {
                defer std.heap.page_allocator.free(content);
                const trimmed = std.mem.trim(u8, content, " \n\t");
                return std.fmt.parseInt(u32, trimmed, 0) catch continue;
            } else |_| {}
        }
        return error.ProductIdNotFound;
    }

    fn readCoreCount(self: *MaliDriver) !u32 {
        _ = self;
        const paths = [_][]const u8{
            "/sys/devices/platform/soc/*/gpu/available_frequencies",
            "/sys/devices/platform/*/mali.*/available_frequencies",
            "/sys/class/misc/mali0/device/available_frequencies",
        };
        for (paths) |path| {
            if (std.fs.cwd().readFileAlloc(path, std.heap.page_allocator, 4096)) |content| {
                defer std.heap.page_allocator.free(content);
                var count: u32 = 0;
                var it = std.mem.splitScalar(u8, content, ' ');
                while (it.next()) |_| {
                    count += 1;
                }
                if (count > 0) return count;
            } else |_| {}
        }
        return self.estimateCoreCount();
    }

    fn estimateCoreCount(self: *MaliDriver) u32 {
        return switch (self.product) {
            .t600 => 1,
            .t620 => 2,
            .t720 => 2,
            .t760 => 4,
            .t820 => 2,
            .t860 => 4,
            .t880 => 8,
            .g31 => 1,
            .g51 => 1,
            .g52 => 2,
            .g71 => 1,
            .g72 => 2,
            .g76 => 4,
            .g77 => 7,
            .g78 => 13,
            .g710 => 10,
            .g715 => 7,
            .g720 => 10,
        };
    }

    fn detectHardwareIssues(self: *MaliDriver) void {
        if (self.architecture == .bifrost) {
            self.hw_issues.append(0x0001) catch {};
        }
        if (self.product == .g71) {
            self.hw_issues.append(0x0002) catch {};
            self.hw_issues.append(0x0003) catch {};
        }
        if (self.product == .g52) {
            self.hw_issues.append(0x0004) catch {};
        }
        if (self.architecture == .valhall and self.product_id < 0x09000071) {
            self.hw_issues.append(0x0005) catch {};
        }
    }

    pub fn hasIssue(self: *const MaliDriver, issue: u32) bool {
        for (self.hw_issues.items) |hw_issue| {
            if (hw_issue == issue) return true;
        }
        return false;
    }

    pub fn getL2CacheSize(self: *const MaliDriver) u32 {
        return switch (self.architecture) {
            .utgard => 0,
            .midgard => 256 * 1024,
            .bifrost => 256 * 1024 * @as(u32, self.num_cores),
            .valhall => 512 * 1024 * @as(u32, self.num_cores),
        };
    }

    pub fn getTilerBinSize(self: *const MaliDriver) u32 {
        return switch (self.architecture) {
            .utgard => 0,
            .midgard => 0,
            .bifrost => 0x10000 * @as(u32, self.num_cores),
            .valhall => 0x20000 * @as(u32, self.num_cores),
        };
    }

    pub fn getMaxTextureSize(self: *const MaliDriver) u32 {
        return switch (self.architecture) {
            .utgard => 4096,
            .midgard => 8192,
            .bifrost => 8192,
            .valhall => 16384,
        };
    }
};
