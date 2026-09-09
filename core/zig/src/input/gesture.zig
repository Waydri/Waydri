const std = @import("std");

pub const GestureState = enum {
    idle,
    pan,
    pinch,
    rotate,
    swipe,
    tap,

    pub fn name(self: GestureState) []const u8 {
        return switch (self) {
            .idle => "Idle",
            .pan => "Pan",
            .pinch => "Pinch",
            .rotate => "Rotate",
            .swipe => "Swipe",
            .tap => "Tap",
        };
    }

    pub fn isActive(self: GestureState) bool {
        return self != .idle;
    }
};

pub const SwipeDirection = enum {
    up,
    down,
    left,
    right,
    up_left,
    up_right,
    down_left,
    down_right,

    pub fn name(self: SwipeDirection) []const u8 {
        return switch (self) {
            .up => "Up",
            .down => "Down",
            .left => "Left",
            .right => "Right",
            .up_left => "Up-Left",
            .up_right => "Up-Right",
            .down_left => "Down-Left",
            .down_right => "Down-Right",
        };
    }

    pub fn isHorizontal(self: SwipeDirection) bool {
        return self == .left or self == .right;
    }

    pub fn isVertical(self: SwipeDirection) bool {
        return self == .up or self == .down;
    }

    pub fn isDiagonal(self: SwipeDirection) bool {
        return self == .up_left or self == .up_right or self == .down_left or self == .down_right;
    }
};

pub const GestureRecognizer = struct {
    state: GestureState,
    start_x: f64,
    start_y: f64,
    current_x: f64,
    current_y: f64,
    last_x: f64,
    last_y: f64,
    pinch_start_distance: f64,
    pinch_scale: f64,
    rotation_angle: f64,
    rotation_accumulated: f64,
    swipe_direction: SwipeDirection,
    tap_count: u32,
    finger_count: u32,
    velocity_x: f64,
    velocity_y: f64,
    threshold: f64,
    pinch_threshold: f64,
    rotate_threshold: f64,
    swipe_threshold: f64,
    last_time_ms: u64,
    callback: ?GestureCallback,

    pub const GestureCallback = *const fn (GestureRecognizer, GestureEvent) void;

    pub const GestureEvent = struct {
        state: GestureState,
        x: f64,
        y: f64,
        delta_x: f64,
        delta_y: f64,
        scale: f64,
        rotation: f64,
        velocity_x: f64,
        velocity_y: f64,
        finger_count: u32,
        swipe_direction: SwipeDirection,
    };

    pub fn init() GestureRecognizer {
        return .{
            .state = .idle,
            .start_x = 0,
            .start_y = 0,
            .current_x = 0,
            .current_y = 0,
            .last_x = 0,
            .last_y = 0,
            .pinch_start_distance = 0,
            .pinch_scale = 1.0,
            .rotation_angle = 0,
            .rotation_accumulated = 0,
            .swipe_direction = .up,
            .tap_count = 0,
            .finger_count = 0,
            .velocity_x = 0,
            .velocity_y = 0,
            .threshold = 10.0,
            .pinch_threshold = 0.1,
            .rotate_threshold = 5.0,
            .swipe_threshold = 100.0,
            .last_time_ms = 0,
            .callback = null,
        };
    }

    pub fn setCallback(self: *GestureRecognizer, callback: GestureCallback) void {
        self.callback = callback;
    }

    pub fn setThresholds(self: *GestureRecognizer, pan: f64, pinch: f64, rotate: f64, swipe: f64) void {
        self.threshold = pan;
        self.pinch_threshold = pinch;
        self.rotate_threshold = rotate;
        self.swipe_threshold = swipe;
    }

    pub fn onTouchDown(self: *GestureRecognizer, x: f64, y: f64, finger: u32, time_ms: u64) void {
        if (finger == 0) {
            self.start_x = x;
            self.start_y = y;
            self.last_x = x;
            self.last_y = y;
            self.current_x = x;
            self.current_y = y;
            self.state = .idle;
            self.tap_count = 0;
            self.pinch_scale = 1.0;
            self.rotation_angle = 0;
            self.rotation_accumulated = 0;
            self.velocity_x = 0;
            self.velocity_y = 0;
        } else if (finger == 1) {
            self.pinch_start_distance = std.math.sqrt(
                (x - self.current_x) * (x - self.current_x) +
                    (y - self.current_y) * (y - self.current_y),
            );
        }
        self.finger_count = finger + 1;
        self.last_time_ms = time_ms;
    }

    pub fn onTouchMove(self: *GestureRecognizer, x: f64, y: f64, finger: u32, time_ms: u64) void {
        if (finger == 0) {
            const dx = x - self.last_x;
            const dy = y - self.last_y;
            self.current_x = x;
            self.current_y = y;

            const total_dx = x - self.start_x;
            const total_dy = y - self.start_y;
            const dist = std.math.sqrt(total_dx * total_dx + total_dy * total_dy);

            if (self.state == .idle and dist > self.threshold) {
                if (self.finger_count >= 3) {
                    const angle = std.math.atan2(total_dy, total_dx);
                    self.rotation_angle = angle;
                    self.state = .rotate;
                } else if (self.finger_count == 2) {
                    self.state = .pinch;
                    self.pinch_scale = 1.0;
                } else {
                    self.state = .pan;
                }
            }

            if (time_ms > self.last_time_ms) {
                const dt = @as(f64, @floatFromInt(time_ms - self.last_time_ms));
                self.velocity_x = dx / dt;
                self.velocity_y = dy / dt;
            }
        }

        if (self.state == .pinch and finger < 2) {
            const dist = std.math.sqrt(
                (self.current_x - x) * (self.current_x - x) +
                    (self.current_y - y) * (self.current_y - y),
            );
            if (self.pinch_start_distance > 0) {
                self.pinch_scale = dist / self.pinch_start_distance;
            }
        }

        if (self.state == .rotate and finger < 3) {
            const current_angle = std.math.atan2(y - self.current_y, x - self.current_x);
            const angle_diff = current_angle - self.rotation_angle;
            self.rotation_accumulated += angle_diff;
            self.rotation_angle = current_angle;
        }

        self.last_time_ms = time_ms;
    }

    pub fn onTouchUp(self: *GestureRecognizer, x: f64, y: f64, finger: u32, time_ms: u64) void {
        if (finger == 0 and self.state == .idle) {
            const dx = x - self.start_x;
            const dy = y - self.start_y;
            const dist = std.math.sqrt(dx * dx + dy * dy);
            if (dist < self.threshold) {
                self.tap_count += 1;
            }
        }

        if (self.finger_count > 0) {
            self.finger_count -= 1;
        }

        if (self.finger_count == 0) {
            if (self.state == .idle and self.tap_count > 0) {
                self.state = .tap;
            } else if (self.state == .idle) {
                const total_dx = x - self.start_x;
                const total_dy = y - self.start_y;
                const dist = std.math.sqrt(total_dx * total_dx + total_dy * total_dy);
                if (dist > self.swipe_threshold) {
                    self.state = .swipe;
                    self.swipe_direction = determineSwipeDirection(total_dx, total_dy);
                }
            }
            self.emitEvent(x, y);
            self.state = .idle;
            self.tap_count = 0;
            self.pinch_scale = 1.0;
            self.rotation_accumulated = 0;
        }

        self.last_time_ms = time_ms;
    }

    fn emitEvent(self: *GestureRecognizer, x: f64, y: f64) void {
        if (self.callback) |cb| {
            cb(self.*, .{
                .state = self.state,
                .x = x,
                .y = y,
                .delta_x = x - self.start_x,
                .delta_y = y - self.start_y,
                .scale = self.pinch_scale,
                .rotation = self.rotation_accumulated,
                .velocity_x = self.velocity_x,
                .velocity_y = self.velocity_y,
                .finger_count = self.finger_count,
                .swipe_direction = self.swipe_direction,
            });
        }
    }

    fn determineSwipeDirection(dx: f64, dy: f64) SwipeDirection {
        const abs_dx = @abs(dx);
        const abs_dy = @abs(dy);
        if (abs_dx > abs_dy) {
            return if (dx > 0) .right else .left;
        } else {
            return if (dy > 0) .down else .up;
        }
    }

    pub fn getState(self: *const GestureRecognizer) GestureState {
        return self.state;
    }

    pub fn getPinchScale(self: *const GestureRecognizer) f64 {
        return self.pinch_scale;
    }

    pub fn getRotationAngle(self: *const GestureRecognizer) f64 {
        return self.rotation_accumulated;
    }

    pub fn getSwipeDirection(self: *const GestureRecognizer) SwipeDirection {
        return self.swipe_direction;
    }

    pub fn isActive(self: *const GestureRecognizer) bool {
        return self.state.isActive();
    }

    pub fn reset(self: *GestureRecognizer) void {
        self.state = .idle;
        self.tap_count = 0;
        self.finger_count = 0;
        self.pinch_scale = 1.0;
        self.rotation_angle = 0;
        self.rotation_accumulated = 0;
        self.velocity_x = 0;
        self.velocity_y = 0;
    }
};
