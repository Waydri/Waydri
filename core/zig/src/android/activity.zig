const std = @import("std");

const c = @cImport({
    @cInclude("android/native_activity.h");
    @cInclude("android/native_window.h");
    @cInclude("android/asset_manager.h");
    @cInclude("android/asset_manager_jni.h");
});

pub const LifecycleState = enum {
    created,
    resumed,
    paused,
    destroyed,
    started,
    stopped,
};

pub const SaveStateResult = struct {
    data: ?[*]u8 = null,
    size: usize = 0,
};

pub const AndroidActivity = struct {
    native_activity: ?*c.ANativeActivity,
    state: LifecycleState,
    asset_manager: ?*c.AAssetManager,
    window: ?*c.ANativeWindow,
    user_data: ?*anyopaque,
    mutex: std.Thread.Mutex,

    pub const OnCreateFn = *const fn (*AndroidActivity) void;
    pub const OnResumeFn = *const fn (*AndroidActivity) void;
    pub const OnPauseFn = *const fn (*AndroidActivity) void;
    pub const OnDestroyFn = *const fn (*AndroidActivity) void;
    pub const OnStartFn = *const fn (*AndroidActivity) void;
    pub const OnStopFn = *const fn (*AndroidActivity) void;
    pub const OnSaveInstanceStateFn = *const fn (*AndroidActivity) SaveStateResult;
    pub const OnWindowFocusChangedFn = *const fn (*AndroidActivity, bool) void;
    pub const OnNativeWindowCreatedFn = *const fn (*AndroidActivity, ?*c.ANativeWindow) void;
    pub const OnNativeWindowDestroyedFn = *const fn (*AndroidActivity, ?*c.ANativeWindow) void;

    on_create: ?OnCreateFn = null,
    on_resume: ?OnResumeFn = null,
    on_pause: ?OnPauseFn = null,
    on_destroy: ?OnDestroyFn = null,
    on_start: ?OnStartFn = null,
    on_stop: ?OnStopFn = null,
    on_save_instance_state: ?OnSaveInstanceStateFn = null,
    on_window_focus_changed: ?OnWindowFocusChangedFn = null,
    on_native_window_created: ?OnNativeWindowCreatedFn = null,
    on_native_window_destroyed: ?OnNativeWindowDestroyedFn = null,

    pub fn init(native: ?*c.ANativeActivity) AndroidActivity {
        return .{
            .native_activity = native,
            .state = .created,
            .asset_manager = if (native) |n| n.assetManager else null,
            .window = null,
            .user_data = null,
            .mutex = .{},
        };
    }

    pub fn deinit(self: *AndroidActivity) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.on_create = null;
        self.on_resume = null;
        self.on_pause = null;
        self.on_destroy = null;
        self.on_start = null;
        self.on_stop = null;
        self.on_save_instance_state = null;
        self.on_window_focus_changed = null;
        self.on_native_window_created = null;
        self.on_native_window_destroyed = null;
        self.window = null;
        self.asset_manager = null;
        self.native_activity = null;
    }

    pub fn onCreated(self: *AndroidActivity) void {
        self.mutex.lock();
        self.state = .created;
        const cb = self.on_create;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self);
    }

    pub fn onResumed(self: *AndroidActivity) void {
        self.mutex.lock();
        self.state = .resumed;
        const cb = self.on_resume;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self);
    }

    pub fn onPaused(self: *AndroidActivity) void {
        self.mutex.lock();
        self.state = .paused;
        const cb = self.on_pause;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self);
    }

    pub fn onDestroyed(self: *AndroidActivity) void {
        self.mutex.lock();
        self.state = .destroyed;
        const cb = self.on_destroy;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self);
    }

    pub fn onStarted(self: *AndroidActivity) void {
        self.mutex.lock();
        self.state = .started;
        const cb = self.on_start;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self);
    }

    pub fn onStopped(self: *AndroidActivity) void {
        self.mutex.lock();
        self.state = .stopped;
        const cb = self.on_stop;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self);
    }

    pub fn onWindowFocusChanged(self: *AndroidActivity, has_focus: bool) void {
        self.mutex.lock();
        const cb = self.on_window_focus_changed;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self, has_focus);
    }

    pub fn onNativeWindowCreated(self: *AndroidActivity, window: ?*c.ANativeWindow) void {
        self.mutex.lock();
        self.window = window;
        const cb = self.on_native_window_created;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self, window);
    }

    pub fn onNativeWindowDestroyed(self: *AndroidActivity, window: ?*c.ANativeWindow) void {
        self.mutex.lock();
        self.window = null;
        const cb = self.on_native_window_destroyed;
        self.mutex.unlock();
        if (cb) |fn_ptr| fn_ptr(self, window);
    }

    pub fn getAssetManager(self: *AndroidActivity) ?*c.AAssetManager {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.asset_manager;
    }

    pub fn getWindow(self: *AndroidActivity) ?*c.ANativeWindow {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.window;
    }

    pub fn getState(self: *AndroidActivity) LifecycleState {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.state;
    }

    pub fn setUserData(self: *AndroidActivity, data: ?*anyopaque) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.user_data = data;
    }

    pub fn getUserData(self: *AndroidActivity) ?*anyopaque {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.user_data;
    }
};
