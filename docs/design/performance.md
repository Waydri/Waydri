# Performance Design

## Frame Scheduling

Waydri targets a configurable frame rate (default 60 FPS). The frame scheduler calculates the budget and sleeps the remaining time:

```
budget_ms = 1000 / max_fps
elapsed_ms = frame_end - frame_start
sleep_ms = budget_ms - elapsed_ms
if sleep_ms > 0 {
    sleep(Duration::from_millis(sleep_ms as u64));
}
```

If a frame exceeds its budget, no sleep is performed and the next frame starts immediately. The frame scheduler tracks frame times and adjusts the target if sustained overruns are detected.

## Damage Tracking

Damage tracking avoids redundant rendering by recording which screen regions have changed since the last frame.

### Aggregation

Incoming damage rectangles are aggregated into a list of non-overlapping regions. Overlapping rectangles are merged to minimize the number of draw calls.

```
damaged = aggregate(damage_queue);
if damaged.is_empty() {
    skip_frame();
    return;
}
```

### Partial Rendering

Only damaged regions are rendered. The renderer clips draw calls to the damaged bounding rectangle, reducing fill rate and bandwidth.

### Full-Screen Damage

Certain operations require full-screen damage:

- Output mode changes (resolution, scale).
- Layout changes affecting all windows.
- Theme or animation changes.
- Plugin full-screen effects.

## Memory Management

### DMA-BUF Import

On Linux, Waydri imports client buffers using DMA-BUF for zero-copy rendering. The compositor reads buffer data directly from GPU memory without CPU-side copies.

### SHM Pools

Shared memory pools provide a fallback for clients that do not support DMA-BUF. Pools use mmap for efficient data transfer between client and compositor.

### Buffer Recycling

Render buffers are recycled using a pool of pre-allocated framebuffers. This avoids allocation overhead during frame rendering.

```
let buffer = buffer_pool.acquire(width, height);
render_to(buffer);
buffer_pool.release(buffer);
```

## GPU Acceleration

### Vulkan Pipeline

The Vulkan renderer uses:

- Pre-compiled shader pipelines for common operations.
- Command buffer pre-recording for static scenes.
- Descriptor set caching to reduce GPU state changes.
- Push constants for per-draw uniform updates.

### GLES Pipeline

The GLES renderer uses:

- Framebuffer objects for off-screen rendering.
- Texture atlases to reduce bind calls.
- Uniform buffer objects for shared shader data.
- Instanced rendering for repeated elements.

## Profiling

### Built-in Metrics

Waydri exposes performance metrics through the IPC interface:

```json
{
    "fps": 60.0,
    "frame_time_ms": 8.2,
    "render_time_ms": 4.1,
    "damage_regions": 3,
    "memory_rss_kb": 128000,
    "memory_vm_kb": 256000
}
```

### External Profiling

- `tools/profile/perf`: CPU and frame time profiling.
- `tools/profile/memory`: RSS and VmSize tracking.
- `perf stat` and `perf record` for low-level profiling.
- `renderdoc` for GPU frame captures.
