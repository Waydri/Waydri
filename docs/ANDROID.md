# Waydri on Android

Waydri runs as an Android application that hosts the Wayland compositor. This document describes the Android-specific integration.

## Immersive Mode

The Android activity uses immersive fullscreen mode, hiding the system bars for a distraction-free compositor surface. Sticky immersive mode is enabled so system bars reappear only on an edge swipe.

## JNI Bridge

The Android app is a thin Java/Kotlin shell. The native Waydri core is compiled to a shared library and invoked through the Java Native Interface (JNI).

- `com.waydri.MainActivity` - the entry activity
- `libwaydri.so` - the native compositor shared library
- JNI functions exposed for lifecycle, input, and surface management

The native library is built with the cross-compile tooling:

```
tools/build/cross_compile
```

## APK Structure

The release APK contains:

- `classes.dex` - the Android app bytecode
- `lib/arm64-v8a/libwaydri.so` - the native compositor
- `assets/config/` - bundled default configuration and themes
- `res/` - Android resources, icons, and layout XML
- `AndroidManifest.xml` - app manifest

## Service Lifecycle

The compositor runs as a foreground service so it continues across activity recreation and can render to the display independently.

- `onCreate`: load configuration, bind IPC socket.
- `onStartCommand`: launch the native compositor thread.
- `onSurfaceCreated`: hand the Android surface handle to the renderer.
- `onDestroy`: signal shutdown and join the compositor thread.

Lifecycle changes (pause/resume) toggle the compositor between suspended and running states to conserve power.

## Building

```
tools/build/android_build debug
tools/build/android_build release
```

For a signed release APK:

```
tools/packaging/apk_build
```

## Testing on Device

```
tools/debug/android_debug
```

This installs the debug APK, starts the compositor, and tails `Waydri*` logcat output.

## Android-Specific Configuration

```toml
[android]
immersive = true
edge_swipe_back = true
keep_screen_on = true
```

These options control the Android window flags and edge gestures.
