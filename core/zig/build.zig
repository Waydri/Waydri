const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "waydri-core",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.linkSystemLibrary("wayland-server");
    exe.linkSystemLibrary("EGL");
    exe.linkSystemLibrary("GL");
    exe.linkSystemLibrary("gbm");
    exe.linkSystemLibrary("drm");
    exe.linkSystemLibrary("vulkan");
    exe.linkSystemLibrary("X11");
    exe.linkLibC();

    const android_mod = b.addModule("android", .{
        .root_source_file = b.path("src/android/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const display_mod = b.addModule("display", .{
        .root_source_file = b.path("src/display/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const driver_mod = b.addModule("driver", .{
        .root_source_file = b.path("src/driver/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const gpu_mod = b.addModule("gpu", .{
        .root_source_file = b.path("src/gpu/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const hal_mod = b.addModule("hal", .{
        .root_source_file = b.path("src/hal/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const input_mod = b.addModule("input", .{
        .root_source_file = b.path("src/input/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const memory_mod = b.addModule("memory", .{
        .root_source_file = b.path("src/memory/mod.zig"),
        .target = target,
        .optimize = optimize,
    });
    const sync_mod = b.addModule("sync", .{
        .root_source_file = b.path("src/sync/mod.zig"),
        .target = target,
        .optimize = optimize,
    });

    exe.root_module.addImport("android", android_mod);
    exe.root_module.addImport("display", display_mod);
    exe.root_module.addImport("driver", driver_mod);
    exe.root_module.addImport("gpu", gpu_mod);
    exe.root_module.addImport("hal", hal_mod);
    exe.root_module.addImport("input", input_mod);
    exe.root_module.addImport("memory", memory_mod);
    exe.root_module.addImport("sync", sync_mod);

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run Waydri core");
    run_step.dependOn(&run_cmd.step);
}
