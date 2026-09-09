# Waydri Performance Guide

This document covers performance tuning for the Waydri compositor, including GPU acceleration, frame scheduling, memory management, and profiling.

## GPU Acceleration

Waydri uses a Vulkan renderer with a GLES fallback. The renderer is selected at startup based on available backend support.

Select the renderer:

```
waydri --renderer vulkan
waydri --renderer gles
```

Enable GPU-accelerated damage tracking where supported by the backend.

### VBlank Sync

Waydri schedules frames against the monitor vblank. Enable vsync to avoid tearing:

```toml
[general]
vsync = true
```

### FPS Cap

Cap the frame rate in the config:

```toml
[general]
max_fps = 144
```

## Frame Scheduling

- The compositor polls for new damage each event-loop iteration.
- Only damaged regions are re-rendered; the rest of the framebuffer is reused.
- Buffer damage is batched and committed in a single swapchain present.
- Frames are budgeted against `max_fps` and vblank to avoid wasted presents.

To verify scheduling, run with frame timing logs:

```
waydri --log fps
```

## Memory Management

- Subsurfaces and non-mapped surfaces are freed promptly on client disconnect.
- Dmabuf buffers are imported and retained with reference counting.
- The damage region tracker is reset per frame to avoid unbounded growth.

Monitor memory usage with the built-in benchmark:

```
cargo bench --bench memory_allocation
cargo bench --bench memory_buffer
```

## Profiling

Generate a flamegraph of CPU usage:

```
tools/profile/flamegraph
```

Record and summarize perf data:

```
tools/profile/cpu perf
```

The flamegraph and perf tools report the top functions by self-time so hot paths can be identified and optimized.

### Benchmarking

Benchmarks live under `benches/` and cover compositor events, input, memory, renderer, and startup paths.

Run all benchmarks:

```
tools/bench/memory
cargo bench --all
```

## Recommended Config

```toml
[general]
max_fps = 144
vsync = true

[render]
backend = "vulkan"
damage_tracking = true
```
