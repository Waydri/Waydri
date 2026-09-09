# Waydri Android Build Guide

This guide describes how to build the Waydri Android APK from source.

## Prerequisites

- Android NDK r26 or newer (download from Android Studio SDK Manager or https://developer.android.com/ndk)
- JDK 17 or newer (OpenJDK recommended)
- Gradle 8.7 or newer
- Android SDK with platform 34
- Rust toolchain with the `aarch64-linux-android` target installed

To add the Android Rust target:

```
rustup target add aarch64-linux-android
```

## Setup

Set the `ANDROID_HOME` environment variable to your SDK location:

```
export ANDROID_HOME=$HOME/Android/Sdk
```

Optionally set `ANDROID_NDK_HOME` if the NDK is installed outside the default location:

```
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/26.1.10909125
```

## Build Commands

### Debug Build

```
tools/build/android_build debug
```

This produces an unsigned, debuggable APK at `output/android/waydri-v0.11.0-debug.apk`.

### Release Build

```
tools/build/android_build release
```

This produces a release APK at `output/android/waydri-v0.11.0-release.apk`.

### Signed APK

To produce a signed and aligned APK, use the packaging script:

```
tools/packaging/apk_build
```

Place a signed keystore at `android/keystore.jks` before running this script. The script signs with the alias `waydri-key` and aligns the result with `zipalign`.

## Cross-Compiling Native Libraries

The Waydri core is written in Rust and compiled to the `aarch64-linux-android` target:

```
tools/build/cross_compile
```

The resulting `.so` files are copied to `output/android-libs/`. The Gradle build links these libraries into the APK's native libraries.

## Testing on Device

Connect a device with USB debugging enabled:

```
tools/debug/android_debug
```

This installs the debug APK, starts the compositor, and streams `Waydri*` logcat tags. If `gdb` is available it also sets up a remote debugging session.

Manual install:

```
adb install -r -t output/android/waydri-v0.11.0-debug.apk
```

## Common Issues

- **NDK not found**: Verify `ANDROID_NDK_HOME` points to an NDK r26+ installation. The cross-compile script falls back to `$ANDROID_HOME/Sdk/ndk/26.1.10909125`.

- **Gradle no daemon**: The build scripts pass `--no-daemon` to avoid leaving background processes. Remove the flag if you prefer a persistent daemon.

- **Missing Rust target**: Run `rustup target add aarch64-linux-android` if cargo reports the target is not installed.

- **JDK version mismatch**: Waydri requires JDK 17+. Use `java -version` to check and update your `JAVA_HOME`.

- **Linker errors**: Set `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER` to the `aarch64-linux-android35-clang` binary if the cross-compile script cannot find it.
