const std = @import("std");

const c = @cImport({
    @cInclude("android/log.h");
    @cInclude("jni.h");
});

pub const LogLevel = enum(c_int) {
    verbose = c.ANDROID_LOG_VERBOSE,
    debug = c.ANDROID_LOG_DEBUG,
    info = c.ANDROID_LOG_INFO,
    warn = c.ANDROID_LOG_WARN,
    error = c.ANDROID_LOG_ERROR,
    fatal = c.ANDROID_LOG_FATAL,
    silent = c.ANDROID_LOG_SILENT,

    pub fn toNative(self: LogLevel) c_int {
        return @intFromEnum(self);
    }
};

pub const Ndk = struct {
    app_name: [*:0]const u8,
    jvm: ?*c.JavaVM,
    jni_env: ?*c.JNIEnv,

    pub fn init(app_name: [*:0]const u8) Ndk {
        return .{
            .app_name = app_name,
            .jni_env = null,
            .jvm = null,
        };
    }

    pub fn initWithJvm(app_name: [*:0]const u8, jvm: ?*c.JavaVM) Ndk {
        return .{
            .app_name = app_name,
            .jni_env = null,
            .jvm = jvm,
        };
    }

    pub fn attachThread(self: *Ndk) !void {
        if (self.jvm) |vm| {
            var env: ?*c.JNIEnv = null;
            const result = vm.*.attachCurrentThread.?(vm, @ptrCast(&env), null);
            if (result != c.JNI_OK) return error.AttachFailed;
            self.jni_env = env;
        } else {
            return error.NoJvm;
        }
    }

    pub fn detachThread(self: *Ndk) void {
        if (self.jvm) |vm| {
            _ = vm.*.detachCurrentThread.?(vm);
            self.jni_env = null;
        }
    }

    pub fn getJNIEnv(self: *const Ndk) ?*c.JNIEnv {
        return self.jni_env;
    }

    pub fn getJvm(self: *const Ndk) ?*c.JavaVM {
        return self.jvm;
    }

    pub fn log(self: *const Ndk, level: LogLevel, comptime fmt: []const u8, args: anytype) void {
        const msg = std.fmt.allocPrintZ(std.heap.page_allocator, fmt, args) catch return;
        defer std.heap.page_allocator.free(msg);
        _ = c.__android_log_print(level.toNative(), self.app_name, "%s", msg.ptr);
    }

    pub fn logVerbose(self: *const Ndk, comptime fmt: []const u8, args: anytype) void {
        self.log(.verbose, fmt, args);
    }

    pub fn logDebug(self: *const Ndk, comptime fmt: []const u8, args: anytype) void {
        self.log(.debug, fmt, args);
    }

    pub fn logInfo(self: *const Ndk, comptime fmt: []const u8, args: anytype) void {
        self.log(.info, fmt, args);
    }

    pub fn logWarn(self: *const Ndk, comptime fmt: []const u8, args: anytype) void {
        self.log(.warn, fmt, args);
    }

    pub fn logError(self: *const Ndk, comptime fmt: []const u8, args: anytype) void {
        self.log(.error, fmt, args);
    }

    pub fn logFatal(self: *const Ndk, comptime fmt: []const u8, args: anytype) void {
        self.log(.fatal, fmt, args);
    }

    pub fn findClass(self: *const Ndk, class_name: [*:0]const u8) !type {
        const env = self.jni_env orelse return error.NoJNIEnv;
        const result = env.*.FindClass.?(env, class_name);
        if (result == null) return error.ClassNotFound;
        return std.meta.Child(@TypeOf(result.?));
    }

    pub fn getStringUtf(self: *const Ndk, jstr: jstring) ![:0]const u8 {
        const env = self.jni_env orelse return error.NoJNIEnv;
        const result = env.*.GetStringUTFChars.?(env, jstr, null);
        if (result == null) return error.StringConversionFailed;
        const slice = std.mem.span(result);
        return slice;
    }

    pub fn newStringUtf(self: *const Ndk, str: [:0]const u8) !jstring {
        const env = self.jni_env orelse return error.NoJNIEnv;
        const result = env.*.NewStringUTF.?(env, str.ptr);
        return result orelse error.StringCreationFailed;
    }

    pub fn callVoidMethod(self: *const Ndk, obj: jobject, method_id: jmethodID, args: ...) void {
        const env = self.jni_env orelse return;
        env.*.CallVoidMethodV.?(env, obj, method_id, @vaStart(args));
    }

    pub fn callIntMethod(self: *const Ndk, obj: jobject, method_id: jmethodID, args: ...) c_int {
        const env = self.jni_env orelse return -1;
        return env.*.CallIntMethodV.?(env, obj, method_id, @vaStart(args));
    }

    pub fn getException(self: *const Ndk) bool {
        const env = self.jni_env orelse return false;
        return env.*.ExceptionCheck.?(env) != 0;
    }

    pub fn clearException(self: *const Ndk) void {
        const env = self.jni_env orelse return;
        env.*.ExceptionClear.?(env);
    }
};

const jstring = ?*opaque {};
const jobject = ?*opaque {};
const jmethodID = ?*const opaque {};
