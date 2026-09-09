# Debugging Guide

## Logging

### RUST_LOG Environment Variable

Control log verbosity with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug waydri
RUST_LOG=waydri=trace waydri
RUST_LOG=waydri_core::renderer=debug waydri
```

### Log Levels

- `error`: Fatal errors that prevent operation.
- `warn`: Recoverable issues that may indicate problems.
- `info`: Normal operational messages (window creation, layout changes).
- `debug`: Detailed operational data (frame timing, damage regions).
- `trace`: Verbose data for deep debugging (protocol messages, input events).

### Filtering by Module

```bash
RUST_LOG=waydri_core::input=trace waydri
RUST_LOG=waydri_core::renderer=debug,waydri_core::window=info waydri
```

## Logcat on Android

```bash
adb logcat -s waydri
adb logcat -s waydri:D
adb logcat -s waydri:V
```

For comprehensive logging:

```bash
adb logcat | grep -i waydri
```

## GDB Attachment

### Attach to Running Process

```bash
adb shell ps | grep waydri
adb shell gdbserver :5039 --attach <pid>
adb forward tcp:5039 tcp:5039
gdb -ex "target remote :5039" -ex "set solib-search-path core/target/debug"
```

### Local Debugging

```bash
RUST_LOG=debug gdb --args target/debug/waydri
(gdb) run
(gdb) break waydri_core::renderer::vulkan::render_frame
(gdb) continue
```

## Render Debug Overlays

Enable visual debug overlays to inspect compositor state:

```bash
RUST_LOG=waydri=debug waydri
```

### Damage Overlay

Shows damaged regions in red. Enabled with `debug.show_damage=true`.

### FPS Counter

Displays frame rate in the corner. Enabled with `debug.show_fps=true`.

### Window Borders

Shows window geometry with colored borders. Enabled with `debug.show_borders=true`.

### Input Events

Logs input events to the screen. Enabled with `debug.show_input=true`.

## Performance Profiling

### Built-in Metrics

Query metrics via IPC:

```bash
echo '{"type":"command","id":"1","command":"get_version"}' | socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/waydri/ipc.sock
```

### External Tools

- `perf record -g -- target/debug/waydri`: CPU profiling.
- `valgrind --tool=memcheck target/debug/waydri`: Memory leak detection.
- `renderdoc`: GPU frame capture (requires Vulkan backend).

## Common Debug Scenarios

### Window Not Appearing

1. Check window creation in logs: `RUST_LOG=waydri_core::window=debug`.
2. Verify the layout is assigning a valid rectangle.
3. Check if the window is on the correct workspace.

### Animation Stuck

1. Check animation state in logs.
2. Verify the frame scheduler is calling update.
3. Check for NaN in animation progress values.

### Input Not Working

1. Check device enumeration: `RUST_LOG=waydri_core::input=trace`.
2. Verify device permissions.
3. Check if another compositor is capturing input.
