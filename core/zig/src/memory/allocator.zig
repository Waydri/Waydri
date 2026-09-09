const std = @import("std");

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
};

pub const AllocatorError = error{
    AllocationFailed,
    InvalidSize,
    InvalidAlignment,
    MapFailed,
    UnmapFailed,
    FreeFailed,
    NotAllocated,
    InvalidBuffer,
};

pub const MemoryAllocator = struct {
    backend: Backend,
    total_allocated: u64,
    allocation_count: u32,
    max_allocations: u32,
    max_total_size: u64,

    pub const Backend = enum {
        gralloc,
        shm,
        dmabuf,
        custom,
    };

    pub fn init(backend: Backend, max_allocations: u32, max_total_size: u64) MemoryAllocator {
        return .{
            .backend = backend,
            .total_allocated = 0,
            .allocation_count = 0,
            .max_allocations = max_allocations,
            .max_total_size = max_total_size,
        };
    }

    pub fn allocateBuffer(self: *MemoryAllocator, width: u32, height: u32, format: BufferFormat, usage: UsageFlags) AllocatorError!BufferHandle {
        if (self.allocation_count >= self.max_allocations) return error.AllocationFailed;

        const stride = self.calculateStride(width, format);
        const size = @as(u64, stride) * @as(u64, height);
        if (self.total_allocated + size > self.max_total_size) return error.AllocationFailed;

        self.total_allocated += size;
        self.allocation_count += 1;

        return .{
            .id = self.allocation_count,
            .width = width,
            .height = height,
            .format = format,
            .stride = stride,
            .size = size,
            .usage = usage,
            .vaddr = null,
            .is_mapped = false,
        };
    }

    pub fn freeBuffer(self: *MemoryAllocator, handle: *BufferHandle) AllocatorError!void {
        if (handle.id == 0) return error.InvalidBuffer;
        if (handle.is_mapped) {
            try self.unmap(handle);
        }
        self.total_allocated -|= handle.size;
        self.allocation_count -|= 1;
        handle.id = 0;
        handle.size = 0;
    }

    pub fn map(self: *MemoryAllocator, handle: *BufferHandle) AllocatorError![]u8 {
        if (handle.id == 0) return error.InvalidBuffer;
        if (handle.is_mapped) return error.MapFailed;
        if (handle.size == 0) return error.InvalidSize;

        var buf = try std.heap.page_allocator.alloc(u8, @intCast(handle.size));
        handle.vaddr = buf.ptr;
        handle.is_mapped = true;
        return buf;
    }

    pub fn unmap(self: *MemoryAllocator, handle: *BufferHandle) AllocatorError!void {
        if (handle.id == 0) return error.InvalidBuffer;
        if (!handle.is_mapped) return error.UnmapFailed;
        if (handle.vaddr) |vaddr| {
            const slice: []u8 = @ptrCast(@alignCast(vaddr));
            std.heap.page_allocator.free(slice[0..@intCast(handle.size)]);
            handle.vaddr = null;
            handle.is_mapped = false;
        }
    }

    pub fn isAvailable(self: *const MemoryAllocator) bool {
        return self.allocation_count < self.max_allocations and
            self.total_allocated < self.max_total_size;
    }

    pub fn getStats(self: *const MemoryAllocator) Stats {
        return .{
            .total_allocated = self.total_allocated,
            .allocation_count = self.allocation_count,
            .max_total_size = self.max_total_size,
            .max_allocations = self.max_allocations,
            .utilization = if (self.max_total_size > 0)
                @as(f64, @floatFromInt(self.total_allocated)) / @as(f64, @floatFromInt(self.max_total_size))
            else
                0,
        };
    }

    pub const UsageFlags = packed struct(u64) {
        gpu_render: bool = false,
        cpu_read: bool = false,
        cpu_write: bool = false,
        video: bool = false,
        camera: bool = false,
        composit: bool = false,
        _padding: u58 = 0,
    };

    pub const BufferHandle = struct {
        id: u32,
        width: u32,
        height: u32,
        format: BufferFormat,
        stride: u32,
        size: u64,
        usage: UsageFlags,
        vaddr: ?*anyopaque,
        is_mapped: bool,

        pub fn isValid(self: BufferHandle) bool {
            return self.id != 0;
        }

        pub fn isAllocated(self: BufferHandle) bool {
            return self.id != 0 and self.size > 0;
        }
    };

    pub const Stats = struct {
        total_allocated: u64,
        allocation_count: u32,
        max_total_size: u64,
        max_allocations: u32,
        utilization: f64,
    };

    fn calculateStride(self: *MemoryAllocator, width: u32, format: BufferFormat) u32 {
        _ = self;
        const bpp = format.bytesPerPixel();
        const alignment: u32 = 64;
        const raw = width * bpp;
        return (raw + alignment - 1) & ~(alignment - 1);
    }
};
