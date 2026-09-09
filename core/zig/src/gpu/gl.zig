const std = @import("std");

const c = @cImport({
    @cInclude("GLES3/gl3.h");
    @cInclude("GLES3/gl3ext.h");
    @cInclude("EGL/egl.h");
});

pub const GlError = error{
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    Unknown,
};

pub const ShaderType = enum(c_uint) {
    vertex = c.GL_VERTEX_SHADER,
    fragment = c.GL_FRAGMENT_SHADER,

    pub fn name(self: ShaderType) []const u8 {
        return switch (self) {
            .vertex => "vertex",
            .fragment => "fragment",
        };
    }
};

pub const Shader = struct {
    id: c_uint,
    shader_type: ShaderType,

    pub fn compile(shader_type: ShaderType, source: [*:0]const u8) !Shader {
        const id = c.glCreateShader(@intFromEnum(shader_type));
        if (id == 0) return error.OutOfMemory;
        c.glShaderSource(id, 1, @ptrCast(&source), null);
        c.glCompileShader(id);

        var status: c_int = 0;
        c.glGetShaderiv(id, c.GL_COMPILE_STATUS, &status);
        if (status == 0) {
            var log_len: c_int = 0;
            c.glGetShaderiv(id, c.GL_INFO_LOG_LENGTH, &log_len);
            if (log_len > 0) {
                var log_buf: [1024]u8 = undefined;
                var actual_len: c_int = 0;
                c.glGetShaderInfoLog(id, 1024, &actual_len, &log_buf);
            }
            c.glDeleteShader(id);
            return error.InvalidOperation;
        }
        return .{ .id = id, .shader_type = shader_type };
    }

    pub fn delete(self: *Shader) void {
        if (self.id != 0) {
            c.glDeleteShader(self.id);
            self.id = 0;
        }
    }
};

pub const Program = struct {
    id: c_uint,

    pub fn create() Program {
        return .{ .id = c.glCreateProgram() };
    }

    pub fn attachShader(self: Program, shader: Shader) void {
        c.glAttachShader(self.id, shader.id);
    }

    pub fn link(self: Program) !void {
        c.glLinkProgram(self.id);
        var status: c_int = 0;
        c.glGetProgramiv(self.id, c.GL_LINK_STATUS, &status);
        if (status == 0) {
            var log_len: c_int = 0;
            c.glGetProgramiv(self.id, c.GL_INFO_LOG_LENGTH, &log_len);
            if (log_len > 0) {
                var log_buf: [1024]u8 = undefined;
                var actual_len: c_int = 0;
                c.glGetProgramInfoLog(self.id, 1024, &actual_len, &log_buf);
            }
            return error.InvalidOperation;
        }
    }

    pub fn use(self: Program) void {
        c.glUseProgram(self.id);
    }

    pub fn delete(self: *Program) void {
        if (self.id != 0) {
            c.glDeleteProgram(self.id);
            self.id = 0;
        }
    }

    pub fn getUniformLocation(self: Program, name: [*:0]const u8) c_int {
        return c.glGetUniformLocation(self.id, name);
    }

    pub fn getAttribLocation(self: Program, name: [*:0]const u8) c_int {
        return c.glGetAttribLocation(self.id, name);
    }

    pub fn setUniform1f(self: Program, location: c_int, value: f32) void {
        c.glUniform1f(location, value);
    }

    pub fn setUniform2f(self: Program, location: c_int, x: f32, y: f32) void {
        c.glUniform2f(location, x, y);
    }

    pub fn setUniform3f(self: Program, location: c_int, x: f32, y: f32, z: f32) void {
        c.glUniform3f(location, x, y, z);
    }

    pub fn setUniform4f(self: Program, location: c_int, x: f32, y: f32, z: f32, w: f32) void {
        c.glUniform4f(location, x, y, z, w);
    }

    pub fn setUniform1i(self: Program, location: c_int, value: c_int) void {
        c.glUniform1i(location, value);
    }

    pub fn setUniformMatrix4fv(self: Program, location: c_int, count: c_int, transpose: bool, value: [*]const f32) void {
        c.glUniformMatrix4fv(location, count, @intFromBool(transpose), value);
    }
};

pub const GlBindings = struct {
    clear_color: [4]f32,
    viewport: Viewport,
    blend_enabled: bool,
    depth_test_enabled: bool,
    cull_face_enabled: bool,

    pub const Viewport = struct {
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    };

    pub fn init() GlBindings {
        return .{
            .clear_color = .{ 0.0, 0.0, 0.0, 1.0 },
            .viewport = .{ .x = 0, .y = 0, .width = 0, .height = 0 },
            .blend_enabled = false,
            .depth_test_enabled = false,
            .cull_face_enabled = false,
        };
    }

    pub fn clearColor(self: *GlBindings, r: f32, g: f32, b: f32, a: f32) void {
        self.clear_color = .{ r, g, b, a };
        c.glClearColor(r, g, b, a);
    }

    pub fn clear(self: *GlBindings, mask: c_uint) void {
        c.glClear(mask);
    }

    pub fn clearDepth(self: *GlBindings, depth: f64) void {
        c.glClearDepthf(@floatCast(depth));
    }

    pub fn clearStencil(self: *GlBindings, s: c_int) void {
        c.glClearStencil(s);
    }

    pub fn viewport(self: *GlBindings, x: c_int, y: c_int, width: c_int, height: c_int) void {
        self.viewport = .{ .x = x, .y = y, .width = width, .height = height };
        c.glViewport(x, y, width, height);
    }

    pub fn enableBlend(self: *GlBindings) void {
        self.blend_enabled = true;
        c.glEnable(c.GL_BLEND);
        c.glBlendFunc(c.GL_SRC_ALPHA, c.GL_ONE_MINUS_SRC_ALPHA);
    }

    pub fn disableBlend(self: *GlBindings) void {
        self.blend_enabled = false;
        c.glDisable(c.GL_BLEND);
    }

    pub fn enableDepthTest(self: *GlBindings) void {
        self.depth_test_enabled = true;
        c.glEnable(c.GL_DEPTH_TEST);
    }

    pub fn disableDepthTest(self: *GlBindings) void {
        self.depth_test_enabled = false;
        c.glDisable(c.GL_DEPTH_TEST);
    }

    pub fn enableCullFace(self: *GlBindings) void {
        self.cull_face_enabled = true;
        c.glEnable(c.GL_CULL_FACE);
    }

    pub fn disableCullFace(self: *GlBindings) void {
        self.cull_face_enabled = false;
        c.glDisable(c.GL_CULL_FACE);
    }

    pub fn drawArrays(self: *GlBindings, mode: DrawMode, first: c_int, count: c_int) void {
        _ = self;
        c.glDrawArrays(@intFromEnum(mode), first, count);
    }

    pub fn drawElements(self: *GlBindings, mode: DrawMode, count: c_int, indices_type: IndexType, indices: ?*const anyopaque) void {
        _ = self;
        c.glDrawElements(@intFromEnum(mode), count, @intFromEnum(indices_type), indices);
    }

    pub fn texImage2D(self: *GlBindings, target: TextureTarget, level: c_int, internal_format: c_int, width: c_int, height: c_int, border: c_int, format: c_uint, pixel_type: c_uint, data: ?*const anyopaque) void {
        _ = self;
        c.glTexImage2D(@intFromEnum(target), level, internal_format, width, height, border, format, pixel_type, data);
    }

    pub fn texSubImage2D(self: *GlBindings, target: TextureTarget, level: c_int, x_offset: c_int, y_offset: c_int, width: c_int, height: c_int, format: c_uint, pixel_type: c_uint, data: ?*const anyopaque) void {
        _ = self;
        c.glTexSubImage2D(@intFromEnum(target), level, x_offset, y_offset, width, height, format, pixel_type, data);
    }

    pub fn bindTexture(self: *GlBindings, target: TextureTarget, texture: c_uint) void {
        _ = self;
        c.glBindTexture(@intFromEnum(target), texture);
    }

    pub fn genTextures(self: *GlBindings, n: c_int, textures: [*]c_uint) void {
        _ = self;
        c.glGenTextures(n, textures);
    }

    pub fn deleteTextures(self: *GlBindings, n: c_int, textures: [*]const c_uint) void {
        _ = self;
        c.glDeleteTextures(n, textures);
    }

    pub fn activeTexture(self: *GlBindings, texture_unit: c_uint) void {
        _ = self;
        c.glActiveTexture(texture_unit);
    }

    pub fn readPixels(self: *GlBindings, x: c_int, y: c_int, width: c_int, height: c_int, format: c_uint, pixel_type: c_uint, data: ?*anyopaque) void {
        _ = self;
        c.glReadPixels(x, y, width, height, format, pixel_type, data);
    }

    pub fn getError(self: *GlBindings) GlError {
        _ = self;
        return switch (c.glGetError()) {
            c.GL_NO_ERROR => return error.Unknown,
            c.GL_INVALID_ENUM => error.InvalidEnum,
            c.GL_INVALID_VALUE => error.InvalidValue,
            c.GL_INVALID_OPERATION => error.InvalidOperation,
            c.GL_INVALID_FRAMEBUFFER_OPERATION => error.InvalidFramebufferOperation,
            c.GL_OUT_OF_MEMORY => error.OutOfMemory,
            else => error.Unknown,
        };
    }

    pub fn getString(self: *GlBindings, name: c_uint) ?[:0]const u8 {
        _ = self;
        const result = c.glGetString(name);
        if (result == null) return null;
        return std.mem.span(result);
    }

    pub fn getIntegerv(self: *GlBindings, pname: c_uint) c_int {
        _ = self;
        var value: c_int = 0;
        c.glGetIntegerv(pname, &value);
        return value;
    }

    pub fn getFloatv(self: *GlBindings, pname: c_uint) f32 {
        _ = self;
        var value: f32 = 0;
        c.glGetFloatv(pname, &value);
        return value;
    }

    pub const COLOR_BUFFER_BIT: c_uint = c.GL_COLOR_BUFFER_BIT;
    pub const DEPTH_BUFFER_BIT: c_uint = c.GL_DEPTH_BUFFER_BIT;
    pub const STENCIL_BUFFER_BIT: c_uint = c.GL_STENCIL_BUFFER_BIT;

    pub const DrawMode = enum(c_uint) {
        points = c.GL_POINTS,
        lines = c.GL_LINES,
        line_strip = c.GL_LINE_STRIP,
        line_loop = c.GL_LINE_LOOP,
        triangles = c.GL_TRIANGLES,
        triangle_strip = c.GL_TRIANGLE_STRIP,
        triangle_fan = c.GL_TRIANGLE_FAN,
    };

    pub const IndexType = enum(c_uint) {
        unsigned_byte = c.GL_UNSIGNED_BYTE,
        unsigned_short = c.GL_UNSIGNED_SHORT,
        unsigned_int = c.GL_UNSIGNED_INT,
    };

    pub const TextureTarget = enum(c_uint) {
        texture_2d = c.GL_TEXTURE_2D,
        texture_cube_map = c.GL_TEXTURE_CUBE_MAP,
        texture_2d_array = c.GL_TEXTURE_2D_ARRAY,
        texture_3d = c.GL_TEXTURE_3D,
    };
};
