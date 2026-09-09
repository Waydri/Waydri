const std = @import("std");

pub const Buffer = struct {
    id: u32,
    width: u32,
    height: u32,
    format: BufferFormat,
    stride: u32,
    size: u64,
    handle: ?*anyopaque,
    is_mapped: bool,
    vaddr: ?[*]u8,
    ref_count: u32,

    pub const BufferFormat = enum(u32) {
        rgba8888 = 1,
        rgbx8888 = 2,
        rgba_fp16 = 3,
        blob = 4,
        yuv420_888 = 5,
        rgba_1010102 = 6,
        undefined = 0,

        pub fn bytesPerPixel(self: BufferFormat) u32 {
            return switch (self) {
                .rgba8888 => 4,
                .rgbx8888 => 4,
                .rgba_fp16 => 8,
                .blob => 1,
                .yuv420_888 => 3,
                .rgba_1010102 => 4,
                .undefined => 0,
            };
        }

        pub fn hasAlpha(self: BufferFormat) bool {
            return switch (self) {
                .rgba8888, .rgba_fp16, .rgba_1010102 => true,
                else => false,
            };
        }
    };

    pub fn init(id: u32, width: u32, height: u32, format: BufferFormat) Buffer {
        const bpp = format.bytesPerPixel();
        const stride = (width * bpp + 63) & ~@as(u32, 63);
        return .{
            .id = id,
            .width = width,
            .height = height,
            .format = format,
            .stride = stride,
            .size = @as(u64, stride) * @as(u64, height),
            .handle = null,
            .is_mapped = false,
            .vaddr = null,
            .ref_count = 1,
        };
    }

    pub fn isAllocated(self: *const Buffer) bool {
        return self.id != 0 and self.size > 0;
    }

    pub fn map(self: *Buffer) ![]u8 {
        if (self.is_mapped) return error.AlreadyMapped;
        if (!self.isAllocated()) return error.NotAllocated;
        if (self.size == 0) return error.InvalidSize;
        const buf = try std.heap.page_allocator.alloc(u8, @intCast(self.size));
        self.vaddr = buf.ptr;
        self.is_mapped = true;
        return buf;
    }

    pub fn unmap(self: *Buffer) !void {
        if (!self.is_mapped) return error.NotMapped;
        if (self.vaddr) |vaddr| {
            const slice: []u8 = @ptrCast(@alignCast(vaddr));
            std.heap.page_allocator.free(slice[0..@intCast(self.size)]);
            self.vaddr = null;
            self.is_mapped = false;
        }
    }

    pub fn retain(self: *Buffer) void {
        self.ref_count += 1;
    }

    pub fn release(self: *Buffer) bool {
        if (self.ref_count == 0) return false;
        self.ref_count -= 1;
        return self.ref_count == 0;
    }

    pub fn getRefcount(self: *const Buffer) u32 {
        return self.ref_count;
    }

    pub fn getSlice(self: *const Buffer) ?[]const u8 {
        if (self.vaddr) |vaddr| {
            return vaddr[0..@intCast(self.size)];
        }
        return null;
    }

    pub fn getMutSlice(self: *Buffer) ?[]u8 {
        if (self.vaddr) |vaddr| {
            return vaddr[0..@intCast(self.size)];
        }
        return null;
    }

    pub fn fill(self: *Buffer, value: u8) !void {
        if (!self.is_mapped) return error.NotMapped;
        if (self.vaddr) |vaddr| {
            const slice: []u8 = @ptrCast(@alignCast(vaddr));
            @memset(slice[0..@intCast(self.size)], value);
        }
    }

    pub fn copyFrom(self: *Buffer, source: []const u8, offset: u64) !void {
        if (!self.is_mapped) return error.NotMapped;
        if (offset + source.len > self.size) return error.InvalidSize;
        if (self.vaddr) |vaddr| {
            const slice: []u8 = @ptrCast(@alignCast(vaddr));
            @memcpy(slice[offset..][0..source.len], source);
        }
    }

    pub fn calculateOffset(self: *const Buffer, x: u32, y: u32) u64 {
        return @as(u64, y) * @as(u64, self.stride) + @as(u64, x) * @as(u64, self.format.bytesPerPixel());
    }

    pub fn pixelAt(self: *Buffer, x: u32, y: u32) ?[]u8 {
        if (!self.is_mapped or self.vaddr == null) return null;
        const offset = self.calculateOffset(x, y);
        if (offset + self.format.bytesPerPixel() > self.size) return null;
        const vaddr = self.vaddr.?;
        const start = @as(usize, @intCast(offset));
        const bpp = self.format.bytesPerPixel();
        return vaddr[start..][0..bpp];
    }
};
