# Android Setup Guide

## Prerequisites

- Android Studio or the standalone SDK/NDK tools.
- Android device with developer options enabled.
- USB cable or wireless ADB connection.

## ADB Setup

### Enable Developer Options

1. Open Settings on your Android device.
2. Tap "About phone" or "About tablet".
3. Tap "Build number" 7 times to enable developer options.
4. Go back to Settings and open "Developer options".
5. Enable "USB debugging".

### Connect via USB

1. Connect the device via USB cable.
2. On the device, tap "Allow" when the USB debugging prompt appears.
3. Verify the connection: `adb devices`.
4. The device should appear as `device` (not `unauthorized`).

### Connect via Wireless

1. Ensure the device and computer are on the same network.
2. Enable "Wireless debugging" in Developer options.
3. Pair: `adb pair <ip>:<port>` (use the pairing code shown on device).
4. Connect: `adb connect <ip>:<port>`.

## Building the APK

### With Android Studio

1. Open the `android/` directory in Android Studio.
2. Sync Gradle when prompted.
3. Select your device from the run configuration.
4. Click "Run" or use `Shift+F10`.

### With Gradle CLI

```bash
cd android
./gradlew assembleDebug
```

The APK is output to `android/app/build/outputs/apk/debug/`.

### Install on Device

```bash
adb install android/app/build/outputs/apk/debug/app-debug.apk
```

## Pushing to Device

For development, you can push the APK directly:

```bash
adb push android/app/build/outputs/apk/debug/app-debug.apk /data/local/tmp/waydri.apk
adb shell pm install -r /data/local/tmp/waydri.apk
```

## Logcat Filtering

### View Waydri Logs

```bash
adb logcat -s waydri
```

### Filter by Severity

```bash
adb logcat -s waydri:V
```

### View Crash Logs

```bash
adb logcat -s waydri:E,AndroidRuntime:E
```

### Clear Logcat Buffer

```bash
adb logcat -c
```

### Save Logs to File

```bash
adb logcat -s waydri -d > waydri_log.txt
```

## SELinux

If Waydri crashes on startup due to SELinux restrictions:

```bash
adb shell getenforce
```

If it returns `Enforcing`, you may need to run in permissive mode:

```bash
adb shell setenforce 0
```

This disables SELinux until the next reboot. For persistent changes, build with a custom ROM or use Magisk.

## Permissions

Ensure the following permissions are granted:

- `SYSTEM_ALERT_WINDOW`: Required for overlay rendering.
- `WRITE_EXTERNAL_STORAGE`: For saving configuration.
- `READ_EXTERNAL_STORAGE`: For loading themes and plugins.

Grant via ADB:

```bash
adb shell pm grant com.waydri.compositor android.permission.SYSTEM_ALERT_WINDOW
```

## Troubleshooting

### App Crashes Immediately

Check logcat for the crash reason. Common causes:

- Missing NDK libraries in the APK.
- SELinux blocking native library loading.
- Insufficient memory on the device.

### Black Screen

- Check if the GPU driver is compatible.
- Try disabling hardware acceleration in config.
- Verify the surface is created successfully in logs.

### No Touch Input

- Verify the input device is recognized by the system.
- Check for input permission issues in logcat.
- Ensure the compositor is not competing with system UI for input.
