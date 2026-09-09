# Waydri on Linux

This document describes the Linux-specific integration of Waydri, including DRM/KMS, GBM, systemd, udev, and permissions.

## DRM/KMS

On Linux, Waydri renders directly to the display via the Direct Rendering Manager and Kernel Mode Setting (KMS). Modes, connectors, and planes are enumerated from the DRM device (`/dev/dri/card*`).

- Output hotplug is handled through udev events.
- The active CRTC modeset is applied through atomic KMS where available.
- VBlank is used to schedule frame commits.

## GBM

The Generic Buffer Management (GBM) library provides the allocation backend for buffer import/export. Client dmabufs are imported into the GBM surface and composed by the Vulkan/GLES renderer.

Required GBM headers are part of `libgbm-dev`.

## systemd Integration

Waydri ships a user service unit (`linux/etc/systemd/user/waydri.service`):

```ini
[Unit]
Description=Waydri Compositor

[Service]
ExecStart=/usr/bin/waydri
Restart=on-failure

[Install]
WantedBy=default.target
```

Enable and start it per-user:

```
systemctl --user enable waydri
systemctl --user start waydri
```

## udev

A udev rule grants the compositor access to input and DRM devices. Install under `/etc/udev/rules.d/`:

```
KERNEL=="card*", SUBSYSTEM=="drm", TAG+="uaccess"
KERNEL=="event*", SUBSYSTEM=="input", TAG+="uaccess"
```

The `uaccess` tag lets the active seat (`uaccess` group) use the devices without broad permissions.

## Permissions

- The compositor must have access to `/dev/dri/card*` and `/dev/input/event*`.
- The user running Waydri must be in the `video`, `input`, and `seat` (if using seat manager) groups, or use systemd logind session device grants.
- The IPC socket is created under `$XDG_RUNTIME_DIR` with 0600 permissions, accessible only to the owning user.
- The Wayland socket is created under `$XDG_RUNTIME_DIR` and marked with `WAYLAND_DISPLAY`.

## Platform Selection

Waydri selects the DRM/KMS platform by default on Linux. It can fall back to headless or X11 platforms for nested development.

```
waydri --platform drm
waydri --platform x11
waydri --platform headless
```

## Configuration Paths on Linux

- User config: `~/.config/waydri/`
- System defaults: `/etc/xdg/waydri/`
- Packaged defaults: `/usr/share/waydri/default_config.toml`
