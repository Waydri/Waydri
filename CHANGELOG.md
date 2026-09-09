# Changelog

All notable changes to Waydri are documented in this file.
The format is based on Keep a Changelog and this project adheres to
Semantic Versioning.

## [Unreleased]

- Schedule frames through the event queue processor.
- Expose IPC commands for layout and workspace control.

## [0.11.0] - 2026-09-09

### Added

- Self-contained Rust compositor core with software framebuffer.
- Hardware rendering detection that prefers Vulkan, falls back to GLES,
  then software.
- Output management with headless outputs, modes, transforms, and scale.
- Window management with focus ordering, tiling trees, floating layers,
  fullscreen state, and window groups.
- Workspace manager with up to five workspaces and per-workspace layouts.
- Layout engine with master-stack, dwindle, grid, custom, and dynamic modes.
- Animation engine with curves, keyframes, springs, and transitions.
- Effects engine for blur, gradient, shadow, opacity, and rounded corners.
- Config loader for TOML and JSON with schema validation.
- Logger sinks for console, file, Android logcat, and syslog.
- IPC server over a Unix socket with JSON commands and events.
- Lua default configuration and JSON themes.
- Android immersive fullscreen activity, service, AIDL, and JNI bridge.
- Zig hardware layer for display, GPU, HAL, memory, and sync.
- XWayland C layer for clipboard, selection, and window management.
- Tests, benchmarks, tools, and documentation.

### Changed

- Renderer backend detection order: Vulkan first, GLES second, software last.
- Main activity enters immersive mode and hides system bars.

### Fixed

- Removed the platform-specific logger link requirement on Android.
- Removed bindings that were unavailable on Android targets.

## [0.10.0] - 2025-11-01

### Added

- Initial project scaffold with module layout.