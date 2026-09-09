# Architecture

## Overview

Waydri is a Wayland compositor designed for Android and Linux. The core is written in Rust with a Zig HAL (Hardware Abstraction Layer) for low-level system integration. The Android frontend uses JNI to bridge between Java and the native compositor.

## Module Hierarchy

```
waydri (root crate)
    waydri-core (core compositor logic)
        renderer         Rendering backends (Vulkan, GLES, Software)
        compositor       Wayland compositor state machine
        window           Window lifecycle and state management
        workspace        Virtual desktop management
        layout           Tiling and arrangement algorithms
        input            Keyboard, pointer, and touch handling
        ipc              Unix socket IPC protocol
        config           Configuration loading and validation
        wayland          Wayland protocol implementation
        plugin           Plugin loading and sandboxing
        output           Display output management
        animation        Window transition animations
        effects          Visual effects (blur, shadows, rounded corners)
        utils            Shared utility types and functions
        logger           Structured logging infrastructure
```

## Core (Rust)

The core crate contains all compositor logic. It is platform-independent and runs on both Android and Linux. The crate exposes a public API consumed by the platform-specific frontends.

### Compositor State Machine

The compositor operates as a state machine with the following phases:

1. **Initialize**: Load config, set up Wayland display, bind protocols.
2. **Running**: Process events, render frames, manage windows.
3. **Suspended**: No visible output, reduced frame rate.
4. **Shutdown**: Clean up resources, save state, exit.

### Event Loop

The main event loop integrates Wayland protocol events, input events, timer callbacks, and plugin ticks:

```
loop {
    wayland.dispatch();
    input.dispatch();
    plugins.tick(dt);
    if damage.is_damaged() {
        compositor.compose_frame();
    }
    frame_scheduler.wait_until_next_frame();
}
```

## Renderer

Three rendering backends are supported with automatic fallback:

1. **Vulkan**: Primary backend for maximum performance. Uses DMA-BUF for zero-copy buffer sharing.
2. **GLES 3.2**: Fallback for devices without Vulkan support. Used on most Android devices.
3. **Software**: CPU rendering for headless and testing environments.

The renderer handles window compositing, damage-based partial redraws, effects, and output presentation.

## Zig HAL Layer

The Zig HAL provides hardware abstraction for:

- Memory management (mmap, DMA-BUF, SHM)
- Input device enumeration and polling
- Display output configuration (DRM/KMS)
- GPU driver detection and initialization
- Android NDK integration

The HAL is compiled as a static library and linked into the Rust core.

## Android JNI Bridge

The Android frontend is a standard Android Activity that:

1. Initializes the Wayland display using the Android surface.
2. Bridges touch input from Android to the compositor input system.
3. Manages the Android lifecycle (pause, resume, destroy).
4. Handles permissions and system service access.

## XWayland Compatibility

XWayland support allows running X11 applications under the Wayland compositor:

1. XWayland binary is launched as a child process.
2. An X11 window manager (XWM) manages X11 windows.
3. X11 windows are wrapped in Wayland surfaces.
4. Input events are translated between X11 and Wayland formats.

## IPC Layer

External tools communicate with the compositor via Unix domain sockets. The IPC layer supports:

- Command/response pattern for synchronous operations.
- Event broadcasting for state change notifications.
- JSON serialization for human-readable protocol.
- Connection tracking with automatic cleanup.

## Configuration

Configuration is loaded from Lua scripts and JSON theme files. The config system supports:

- Runtime reloading without compositor restart.
- Per-workspace layout configuration.
- Plugin manifest validation.
- Theme hot-swapping.
