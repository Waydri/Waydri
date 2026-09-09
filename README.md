# Waydri - Wayland Composer

Waydri is a Wayland compositor for Android and Linux that runs on mobile GPUs.
It pairs a self-contained Rust core with a low-level Zig layer for GPU and
HAL access, an Android app shell with immersive fullscreen rendering, and an
XWayland compatibility layer for legacy X11 applications.

## Features

- Self-contained compositor core in Rust with a software framebuffer and a
  hardware backend that prefers Vulkan and falls back to GLES and software.
- Tiling layouts: master-stack, dwindle, grid, custom, and dynamic.
- Window management with floating, fullscreen, and grouping support.
- Multiple workspaces with persistence.
- Animations with configurable curves and springs.
- Effects: blur, gradient, shadow, rounded corners, and opacity.
- Lua configuration: keybinds, layouts, themes, and rules.
- JSON themes and plugin metadata.
- IPC over a Unix socket for commands and events.
- Android application with a Kotlin shell, AIDL service, and JNI bridge.
- Zig layer for display, GPU driver, HAL, memory, and sync primitives.
- XWayland C layer for clipboard, selection, and window management.

## Directory layout

- core/        Rust compositor core and Zig hardware layer
- android/     Android application, JNI, and GLSL shaders
- xwayland/    X11 compatibility layer in C
- config/      Default user configuration
- docs/        Documentation and examples
- tests/       Unit, integration, and protocol tests
- benches/     Rust benchmarks
- tools/       Build, debug, and profiling helpers
- linux/       Linux filesystem layout for the runtime environment

## Building

The Rust core builds with cargo. The crate targets Android ABIs and Linux.

    cd core
    cargo build

The Android application builds with Gradle.

    cd android
    ./gradlew assembleRelease

Lua configuration files are validated against the loader. JSON themes are
validated against the schema in core/src/config/schema.rs.

## License

Licensed under the Apache License, Version 2.0. See LICENSE.
