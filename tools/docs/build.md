# Waydri Build Documentation

This document covers building Waydri for Linux and Android, using the Zig toolchain, and the CI/CD setup.

## Linux Build

### Prerequisites

- Rust toolchain (stable) with `rustup`
- Cargo
- System libraries: `libwayland-dev`, `libxkbcommon-dev`, `libinput-dev`, `libgbm-dev`, `libdrm-dev`
- For Vulkan rendering: `libvulkan-dev`
- For XWayland: `libxcb1-dev`, `libxcb-ewmh-dev`, `libxcb-icccm4-dev`, `libxkbcommon-x11-dev`

Install system dependencies on Debian/Ubuntu:

```
apt-get install -y build-essential pkg-config libwayland-dev libxkbcommon-dev \
    libinput-dev libgbm-dev libdrm-dev libvulkan-dev \
    libxcb1-dev libxcb-ewmh-dev libxcb-icccm4-dev libxkbcommon-x11-dev
```

### Build

```
cargo build --release
```

The release binary is located at `target/release/waydri`.

### Performance Flags

For maximum performance, pass `-D` (release LTO) and `--codegen units` flags:

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

If you want LTO enabled:

```
RUSTFLAGS="-C target-cpu=native -C lto=true" cargo build --release
```

### Testing

```
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Android Build

### Prerequisites

- Android NDK r26+
- JDK 17+
- Gradle 8.7+
- Rust target: `aarch64-linux-android`

### Setup

```
rustup target add aarch64-linux-android
export ANDROID_HOME=$HOME/Android/Sdk
```

### Cross-Compile the Core

```
tools/build/cross_compile
```

This builds `waydri-core` and copies the `.so` files to `output/android-libs/`.

### Build the APK

```
tools/build/android_build debug
tools/build/android_build release
```

For a signed APK:

```
tools/packaging/apk_build
```

## Zig Build

The Zig toolchain is used for certain platform-specific shims and the renderer module.

### Prerequisites

- Zig 0.11.0 or newer

### Build

```
cd core/zig
zig build
```

The output is placed in `core/zig/zig-out/`. The GPU shaders are compiled at build time from GLSL sources.

### Dispatch to Zig Build

To build the Zig components as part of a full Waydri build:

```
zig build --verbose
```

To cross-compile to aarch64 for Android:

```
zig build -Dtarget=aarch64-linux-android
```

## CI/CD

The Waydri CI runs on GitHub Actions.

### Workflow Stages

1. **Lint**: Runs `cargo fmt --check` and `cargo clippy -- -D warnings`.
2. **Test**: Runs `cargo test` on the Linux host.
3. **Build Linux**: Runs `tools/build/linux_build` to produce the release tarball.
4. **Build Android**: Runs `tools/build/android_build release` on a machine with the Android SDK.
5. **Package**: Runs `tools/build/package` to produce checksums and archives.
6. **Publish**: Uploads artifacts to the release page.

### Required CI Variables

- `ANDROID_HOME` - Android SDK path on the build runner
- `ANDROID_NDK_HOME` - Android NDK path
- `JAVA_HOME` - JDK 17+ path

## Optimization Flags Reference

| Flag | Effect |
|------|--------|
| `-C target-cpu=native` | Tune for the host CPU |
| `-C lto=true` | Enable link-time optimization |
| `-C opt-level=3` | Maximum code optimization |
| `-C codegen-units=1` | Single codegen unit for better optimization |
| `-Zbuild-std` | Build the standard library from source |

### Recommended Release Profile

```toml
[profile.release]
lto = "thin"
codegen-units = 1
opt-level = 3
strip = "symbols"
```
