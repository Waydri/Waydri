pub const drm = @import("drm.zig");
pub const egl = @import("egl.zig");
pub const gbm = @import("gbm.zig");
pub const gl = @import("gl.zig");
pub const hwcomposer = @import("hwcomposer.zig");
pub const vulkan = @import("vulkan.zig");

pub const GpuBackend = enum {
    vulkan,
    opengl_es,
    software,

    pub fn name(self: GpuBackend) []const u8 {
        return switch (self) {
            .vulkan => "Vulkan",
            .opengl_es => "OpenGL ES",
            .software => "Software",
        };
    }

    pub fn requiresDriver(self: GpuBackend) bool {
        return self != .software;
    }

    pub fn supportsHdr(self: GpuBackend) bool {
        return self == .vulkan;
    }
};
