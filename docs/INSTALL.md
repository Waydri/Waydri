# Installing Waydri

## Linux Package

Waydri is distributed as a source tree with build tooling. On Debian/Ubuntu systems the required build dependencies are installed with apt. There is no binary package repository yet; install from source or use the release tarball produced by `tools/build/linux_build`.

### System Dependencies

```
apt-get install -y build-essential pkg-config rustup \
    libwayland-dev libxkbcommon-dev libinput-dev libgbm-dev libdrm-dev \
    libvulkan-dev libxcb1-dev libxcb-ewmh-dev libxcb-icccm4-dev \
    libxkbcommon-x11-dev
```

### Install from Source

```
git clone https://github.com/Waydri/Waydri.git
cd Waydri
cargo build --release
sudo cp target/release/waydri /usr/local/bin/
```

### Install with systemd

Copy the user service unit and enable it:

```
cp linux/etc/systemd/user/waydri.service ~/.config/systemd/user/
systemctl --user enable waydri
systemctl --user start waydri
```

## Android

Install the APK on a device with USB debugging enabled:

```
tools/build/android_build debug
adb install -r -t output/android/waydri-v0.11.0-debug.apk
```

For a release build:

```
tools/packaging/apk_build
adb install -r output/android/waydri-v0.11.0.apk
```

The APK requires Android 8.0 (API 26) or newer and targets 64-bit ARM devices.

## Building from Source

See [BUILD.md](BUILD.md) for full build instructions covering the Rust toolchain, Android NDK, Zig, and CMake.

## Dependencies Summary

| Component | Requirement |
|-----------|-------------|
| Rust | stable toolchain, `aarch64-linux-android` target for Android |
| Zig | 0.11.0+ for shader/platform shims |
| CMake | used by the xwayland and native layer build |
| Android NDK | r26+ for Android builds |
| JDK | 17+ for Android builds |
| Gradle | 8.7+ for Android builds |
