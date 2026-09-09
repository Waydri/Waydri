# Performance Tuning

## GPU Acceleration

### Verify GPU Availability

```bash
vulkaninfo | head -20
glxinfo | grep "OpenGL renderer"
```

### Choose Renderer

Waydri auto-selects the best renderer. To force a specific backend:

```bash
WAYDRI_RENDERER=vulkan waydri
WAYDRI_RENDERER=gles waydri
WAYDRI_RENDERER=software waydri
```

### Vulkan Setup (Linux)

1. Install the Vulkan driver for your GPU:
   - NVIDIA: `nvidia-utils`
   - AMD: `vulkan-radeon`
   - Intel: `vulkan-intel`
2. Install `vulkan-loader`.
3. Verify: `vulkaninfo`.

### GLES Setup (Android)

GLES is available by default on Android devices. No additional setup is needed.

## Frame Rate Tuning

Adjust the target frame rate in config:

```lua
return {
    general = {
        max_fps = 60,  -- Default: 60
    },
}
```

For battery-saving on mobile:

```lua
return {
    general = {
        max_fps = 30,
    },
}
```

### Adaptive Frame Rate

When VSync is enabled, Waydri matches the output refresh rate. Disable VSync for benchmarking:

```lua
return {
    general = {
        vsync = false,
    },
}
```

## Memory Limits

### Buffer Pool Size

Control the number of pre-allocated render buffers:

```lua
return {
    renderer = {
        buffer_pool_size = 4,  -- Default: 4
    },
}
```

### SHM Pool Size

For applications using shared memory:

```lua
return {
    wayland = {
        shm_pool_size = 10485760,  -- 10MB default
    },
}
```

### Plugin Memory Limits

Restrict plugin memory usage:

```lua
return {
    plugins = {
        max_memory_kb = 51200,  -- 50MB per plugin
    },
}
```

## Profiling

### Frame Timing

Enable frame timing overlay:

```bash
RUST_LOG=debug waydri 2>&1 | grep "frame_time"
```

### CPU Profiling

```bash
perf record -g --target/debug/waydri
perf report
```

### Memory Profiling

```bash
valgrind --tool=memcheck --leak-check=full target/debug/waydri
```

### Benchmark Suite

```bash
cargo bench --package waydri-core
```

Benchmark results are saved to `benchmark_results.txt`.

## Optimization Tips

1. Disable effects on low-end hardware:

```lua
return {
    effects = {
        blur_enabled = false,
        shadows_enabled = false,
    },
}
```

2. Reduce animation duration:

```lua
return {
    animations = {
        duration = 150,
    },
}
```

3. Use hardware acceleration whenever possible.
4. Minimize window count on constrained devices.
5. Disable XWayland if not needed:

```lua
return {
    xwayland = {
        enabled = false,
    },
}
```
