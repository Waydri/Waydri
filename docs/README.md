# Waydri

Waydri is a Wayland compositor built for Android and Linux. It provides a modern tiling window manager with GPU-accelerated rendering, a plugin system, and an IPC interface for external control.

## Features

- Vulkan-based rendering with GLES fallback
- Tiling window layouts: master stack, grid, dwindle, and custom
- Full Wayland protocol support including layer-shell and primary-selection
- Spring-physics animation system with configurable curves
- Plugin ABI for extensibility
- Unix-socket IPC interface
- Android support with immersive mode and JNI bridge
- XWayland integration on Linux
- Hot-reloadable themes and configurations

## Architecture

Waydri is split into two crates:

- `waydri` - the compositor binary and entry point
- `waydri-core` - the compositor subsystem crate (state, compositor, layout, window, input, ipc)

The compositor core is written in Rust, with a Zig backend used for platform shims and shader compilation. The X11 compatibility layer (XWayland) lives under `xwayland/`.

## Quick Start

### Linux

```
cargo build --release
./target/release/waydri
```

### Android

```
tools/build/android_build debug
adb install -r -t output/android/waydri-v0.11.0-debug.apk
```

See [docs/INSTALL.md](INSTALL.md) for platform-specific setup and [docs/ARCHITECTURE.md](ARCHITECTURE.md) for a deep dive into the internal design.

## Documentation

- [Installation](INSTALL.md)
- [Build Instructions](BUILD.md)
- [Configuration](CONFIG.md)
- [Keybindings](KEYBINDS.md)
- [Layouts](LAYOUTS.md)
- [Themes](THEMES.md)
- [Plugins](PLUGINS.md)
- [Performance](PERFORMANCE.md)
- [Architecture](ARCHITECTURE.md)

## License

Waydri is licensed under the MIT License. See [docs/LICENSE.md](LICENSE.md).
