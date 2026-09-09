# Building Waydri

This document describes the Waydri build system. Waydri is a Rust workspace with a Zig backend and Android Gradle build.

## Rust Toolchain Setup

Install Rust with rustup:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

Add the Android target for cross-compilation (required for Android builds):

```
rustup target add aarch64-linux-android
```

## Linux Build

Build the release binary:

```
cargo build --release
```

The binary lands at `target/release/waydri`.

Use `tools/build/linux_build` to produce a stripped binary and a tarball of config and core contents:

```
tools/build/linux_build
```

## Android SDK and NDK

Install the Android SDK and NDK via Android Studio or the command line. Required:

- Android NDK r26 or newer
- Android SDK Platform 34
- Build Tools
- JDK 17

Set environment variables:

```
export ANDROID_HOME=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/26.1.10909125
```

## Android Build

Cross-compile the core crate:

```
tools/build/cross_compile
```

Build the APK:

```
tools/build/android_build debug
tools/build/android_build release
```

Sign and align:

```
tools/packaging/apk_build
```

## Zig Toolchain

Install Zig 0.11.0 or newer:

```
snap install zig --classic
```

Build the Zig backend components:

```
cd core/zig
zig build
```

Cross-compile the Zig shims to Android:

```
zig build -Dtarget=aarch64-linux-android
```

## CMake

CMake is used by the XWayland compatibility layer and the native shim build. Install and invoke:

```
apt-get install -y cmake ninja-build
cd xwayland
cmake -B build -G Ninja
cmake --build build
```

## Cross-Compilation

The native layer cross-compiles to `aarch64-linux-android` using the NDK toolchain. The `cross_compile` script sets:

- `CC=aarch64-linux-android35-clang`
- `AR=aarch64-linux-android-ar`
- `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` to the clang binary

## Optimization Flags

Waydri ships a tuned release profile in `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

Additional per-command flags:

| Flag | Effect |
|------|--------|
| `-C target-cpu=native` | Tune generated code for the host CPU |
| `-C lto=fat` | Full cross-crate link-time optimization |

Build with native CPU tuning:

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Verification

Run the full test, lint, and format suite:

```
tools/test/run_all
cargo clippy -- -D warnings
cargo fmt -- --check
```
