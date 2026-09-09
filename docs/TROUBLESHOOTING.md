# Troubleshooting

## Black Screen

A black screen after launching Waydri usually indicates a GPU driver issue.

1. Verify your GPU driver is installed and up to date. For NVIDIA, ensure `nvidia-drm.modeset=1` is set in your kernel parameters.
2. Check that your user is in the `video` and `render` groups: `sudo usermod -aG video,render $USER`.
3. Run Waydri with `RUST_LOG=debug waydri` to see which renderer backend is selected.
4. If Vulkan fails, Waydri falls back to GLES, then to software rendering. Check logs for fallback messages.
5. On Android, ensure the GPU driver is not blocked by SELinux: `adb shell getenforce` should return `Permissive` or `Disabled`.

## No Input

If keyboard and mouse input is not reaching Waydri:

1. Check device permissions: `ls -la /dev/input/event*`. Your user needs read access.
2. On Linux, add your user to the `input` group: `sudo usermod -aG input $USER`.
3. Verify no other compositor is running on the same Wayland display.
4. Check that the input devices are recognized: `sudo libinput list-devices`.
5. On Android, ensure touch input is routed to the compositor layer, not the system UI.

## Screen Tearing

Tearing artifacts during window movement or video playback:

1. Enable VSync in your config: set `general.vsync = true`.
2. Check if your GPU supports hardware VSync. Use `RUST_LOG=debug` to check frame scheduling.
3. On NVIDIA, enable `ForceFullCompositionPipeline` in xorg.conf if using XWayland apps.
4. Reduce `max_fps` if your GPU cannot sustain the target frame rate.

## Crashes on Startup

Waydri exits immediately or crashes with a signal:

1. Check the log output: `RUST_LOG=trace waydri 2>&1 | tee waydri.log`.
2. Verify all required dependencies are installed. Check `docs/BUILD.md` for the full list.
3. Ensure the Wayland display socket is available. If running nested, set `WAYLAND_DISPLAY` correctly.
4. Check for conflicting Wayland environment variables: `env | grep WAYLAND`.
5. On Android, check logcat: `adb logcat -s waydri`.

## High CPU Usage

Waydri consuming excessive CPU:

1. Check if animations are stuck in an infinite loop. Disable animations: set `animations.enabled = false`.
2. Reduce `max_fps` in config. Default is 60; try 30 for lower-end hardware.
3. Profile with `tools/profile/perf` to identify the bottleneck.
4. Check if any plugin is consuming excessive ticks in its `on_tick` callback.
5. Ensure damage tracking is working correctly. Excessive redraws indicate a damage tracking bug.

## No XWayland Applications

X11 applications not starting under XWayland:

1. Verify the XWayland binary is installed: `which xwayland` or check `xwayland/bin/`.
2. Set the `XWAYLAND_DISPLAY` environment variable if not using the default.
3. Check that the XWayland socket is created in the runtime directory.
4. Ensure `xwayland` is enabled in config: `xwayland.enabled = true`.
5. For applications requiring XDG_RUNTIME_DIR, set it: `export XWAYLAND_RUNTIME_DIR=/run/user/$(id -u)`.

## Audio Issues

No audio output through PipeWire/PulseAudio:

1. Check PipeWire is running: `systemctl --user status pipewire pipewire-pulse`.
2. Verify the audio output device is selected in your audio settings.
3. On Android, ensure the audio permissions are granted to the compositor.

## Performance Regressions

After an update, performance is worse:

1. Check the changelog for breaking changes: `docs/CHANGELOG.md`.
2. Run `cargo test` to verify no regressions in core logic.
3. Compare benchmark results: `cargo bench --package waydri-core`.
4. Bisect the git history to find the offending commit: `git bisect start`.

## Getting Help

1. Run `waydri --version` and include the output in your bug report.
2. Provide the full log output with `RUST_LOG=trace`.
3. Include your system info: GPU model, driver version, kernel version.
4. Open an issue at https://github.com/Waydri/Waydri/issues.
