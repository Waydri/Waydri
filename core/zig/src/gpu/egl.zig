const std = @import("std");

const c = @cImport({
    @cInclude("EGL/egl.h");
    @cInclude("EGL/eglext.h");
});

pub const EglError = error{
    NotInitialized,
    BadAccess,
    BadAlloc,
    BadConfig,
    BadContext,
    BadCurrentSurface,
    BadDisplay,
    BadMatch,
    BadNativePixmap,
    BadNativeWindow,
    BadParameter,
    BadSurface,
    ContextLost,
    BadAttribute,
    BadNativeDisplay,
    BadStream,
    BadSwapchain,
    Suboptimal,
    SurfaceLost,
    Timeout,
    NotCompatible,
    Unknown,
};

pub const EglConfig = struct {
    id: c.EGLConfig,
    red_size: c_int,
    green_size: c_int,
    blue_size: c_int,
    alpha_size: c_int,
    depth_size: c_int,
    stencil_size: c_int,
    sample_buffers: c_int,
    samples: c_int,
    surface_type: c_int,
    renderable_type: c_int,
    native_visual_id: c_int,

    pub fn getAttrib(self: EglConfig, display: c.EGLDisplay, attribute: c.EGLint) !c.EGLint {
        var value: c.EGLint = 0;
        if (c.eglGetConfigsAttrib(display, self.id, attribute, &value) == c.EGL_FALSE) return error.BadAttribute;
        return value;
    }
};

pub const EglContext = struct {
    display: c.EGLDisplay,
    surface: ?c.EGLSurface,
    context: c.EGLContext,
    config: EglConfig,
    version_major: c_int,
    version_minor: c_int,
    client_api: c_int,

    pub fn init(display: c.EGLDisplay) !EglContext {
        var major: c_int = 0;
        var minor: c_int = 0;
        if (c.eglInitialize(display, &major, &minor) == c.EGL_FALSE) return error.NotInitialized;

        return .{
            .display = display,
            .surface = null,
            .context = c.EGL_NO_CONTEXT,
            .config = .{
                .id = null,
                .red_size = 0,
                .green_size = 0,
                .blue_size = 0,
                .alpha_size = 0,
                .depth_size = 0,
                .stencil_size = 0,
                .sample_buffers = 0,
                .samples = 0,
                .surface_type = 0,
                .renderable_type = 0,
                .native_visual_id = 0,
            },
            .version_major = major,
            .version_minor = minor,
            .client_api = c.eglQueryAPI(),
        };
    }

    pub fn chooseConfig(self: *EglContext, attribs: []const c.EGLint) !EglConfig {
        var config: c.EGLConfig = null;
        var num_configs: c.EGLint = 0;
        if (c.eglChooseConfig(self.display, attribs.ptr, &config, 1, &num_configs) == c.EGL_FALSE) return error.BadConfig;
        if (num_configs == 0) return error.BadConfig;

        var red: c.EGLint = 0;
        var green: c.EGLint = 0;
        var blue: c.EGLint = 0;
        var alpha: c.EGLint = 0;
        var depth: c.EGLint = 0;
        var stencil: c.EGLint = 0;
        var sample_buf: c.EGLint = 0;
        var samples: c.EGLint = 0;
        var surf_type: c.EGLint = 0;
        var render_type: c.EGLint = 0;
        var native_vid: c.EGLint = 0;

        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_RED_SIZE, &red);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_GREEN_SIZE, &green);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_BLUE_SIZE, &blue);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_ALPHA_SIZE, &alpha);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_DEPTH_SIZE, &depth);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_STENCIL_SIZE, &stencil);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_SAMPLE_BUFFERS, &sample_buf);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_SAMPLES, &samples);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_SURFACE_TYPE, &surf_type);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_RENDERABLE_TYPE, &render_type);
        _ = c.eglGetConfigsAttrib(self.display, config, c.EGL_NATIVE_VISUAL_ID, &native_vid);

        self.config = .{
            .id = config,
            .red_size = red,
            .green_size = green,
            .blue_size = blue,
            .alpha_size = alpha,
            .depth_size = depth,
            .stencil_size = stencil,
            .sample_buffers = sample_buf,
            .samples = samples,
            .surface_type = surf_type,
            .renderable_type = render_type,
            .native_visual_id = native_vid,
        };
        return self.config;
    }

    pub fn createContext(self: *EglContext, api: c.EGLint, share_context: ?c.EGLContext, attribs: ?[]const c.EGLint) !c.EGLContext {
        _ = c.eglBindAPI(api);
        const attribs_ptr: ?[*]const c.EGLint = if (attribs) |a| a.ptr else null;
        const ctx = c.eglCreateContext(self.display, self.config.id, share_context, attribs_ptr);
        if (ctx == c.EGL_NO_CONTEXT) return error.BadContext;
        self.context = ctx;
        return ctx;
    }

    pub fn createWindowSurface(self: *EglContext, native_window: ?*anyopaque, attribs: ?[]const c.EGLint) !void {
        const attribs_ptr: ?[*]const c.EGLint = if (attribs) |a| a.ptr else null;
        const win: ?*anyopaque = native_window;
        const surface = c.eglCreateWindowSurface(self.display, self.config.id, @ptrCast(win), attribs_ptr);
        if (surface == c.EGL_NO_SURFACE) return error.BadSurface;
        self.surface = surface;
    }

    pub fn makeCurrent(self: *EglContext) !void {
        const result = c.eglMakeCurrent(self.display, self.surface, self.surface, self.context);
        if (result == c.EGL_FALSE) return error.BadCurrentSurface;
    }

    pub fn swapBuffers(self: *EglContext) !void {
        if (self.surface) |surface| {
            const result = c.eglSwapBuffers(self.display, surface);
            if (result == c.EGL_FALSE) return error.BadSurface;
        } else return error.BadSurface;
    }

    pub fn swapInterval(self: *EglContext, interval: c_int) !void {
        const result = c.eglSwapInterval(self.display, interval);
        if (result == c.EGL_FALSE) return error.BadParameter;
    }

    pub fn destroySurface(self: *EglContext) void {
        if (self.surface) |surface| {
            _ = c.eglDestroySurface(self.display, surface);
            self.surface = null;
        }
    }

    pub fn destroyContext(self: *EglContext) void {
        if (self.context != c.EGL_NO_CONTEXT) {
            _ = c.eglDestroyContext(self.display, self.context);
            self.context = c.EGL_NO_CONTEXT;
        }
    }

    pub fn terminate(self: *EglContext) void {
        self.destroySurface();
        self.destroyContext();
        _ = c.eglTerminate(self.display);
    }

    pub fn getProcAddress(self: *EglContext, name: [*:0]const u8) ?*const anyopaque {
        return c.eglGetProcAddress(name);
    }

    pub fn queryString(self: *EglContext, name: c.EGLint) ?[:0]const u8 {
        const result = c.eglQueryString(self.display, name);
        if (result == null) return null;
        return std.mem.span(result);
    }

    pub fn getError() EglError {
        return switch (c.eglGetError()) {
            c.EGL_SUCCESS => return error.Unknown,
            c.EGL_NOT_INITIALIZED => error.NotInitialized,
            c.EGL_BAD_ACCESS => error.BadAccess,
            c.EGL_BAD_ALLOC => error.BadAlloc,
            c.EGL_BAD_CONFIG => error.BadConfig,
            c.EGL_BAD_CONTEXT => error.BadContext,
            c.EGL_BAD_CURRENT_SURFACE => error.BadCurrentSurface,
            c.EGL_BAD_DISPLAY => error.BadDisplay,
            c.EGL_BAD_MATCH => error.BadMatch,
            c.EGL_BAD_NATIVE_PIXMAP => error.BadNativePixmap,
            c.EGL_BAD_NATIVE_WINDOW => error.BadNativeWindow,
            c.EGL_BAD_PARAMETER => error.BadParameter,
            c.EGL_BAD_SURFACE => error.BadSurface,
            c.EGL_CONTEXT_LOST => error.ContextLost,
            else => error.Unknown,
        };
    }

    pub fn getDisplayAttrib(self: *EglContext, attribute: c.EGLint) !c.EGLint {
        var value: c.EGLint = 0;
        if (c.eglGetConfigAttrib(self.display, self.config.id, attribute, &value) == c.EGL_FALSE) return error.BadAttribute;
        return value;
    }

    pub fn querySurface(self: *EglContext, attribute: c.EGLint) !c.EGLint {
        if (self.surface) |surface| {
            return c.eglQuerySurface(self.display, surface, attribute);
        }
        return error.BadSurface;
    }

    pub const DEFAULT_ATTRIBS = [_]c.EGLint{
        c.EGL_RED_SIZE,   8,
        c.EGL_GREEN_SIZE, 8,
        c.EGL_BLUE_SIZE,  8,
        c.EGL_ALPHA_SIZE, 8,
        c.EGL_DEPTH_SIZE, 24,
        c.EGL_STENCIL_SIZE, 0,
        c.EGL_RENDERABLE_TYPE,
        c.EGL_OPENGL_ES3_BIT,
        c.EGL_SURFACE_TYPE,
        c.EGL_WINDOW_BIT,
        c.EGL_NONE,
    };

    pub const CONTEXT_ATTRIBS_ES3 = [_]c.EGLint{
        c.EGL_CONTEXT_CLIENT_VERSION, 3,
        c.EGL_NONE,
    };
};
