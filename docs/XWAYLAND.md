# XWayland Support

Waydri can run XWayland to host legacy X11 applications alongside native Wayland clients. The XWayland compatibility layer lives under `xwayland/`.

## Setup

Build the XWayland layer with CMake:

```
apt-get install -y libxcb1-dev libxcb-ewmh-dev libxcb-icccm4-dev xwayland
cd xwayland
cmake -B build -G Ninja
cmake --build build
```

Enable XWayland in the config:

```toml
[xwayland]
enabled = true
display = 0
```

Waydri spawns an XWayland server and exposes its display to the session. X clients connect via `DISPLAY=:0` while Wayland clients use `WAYLAND_DISPLAY`.

## X11 Window Management

X11 windows are managed through the XWM component, which:

- Tracks X windows and their properties.
- Applies EWMH window state.
- Bridges map/unmap, focus, and input to the Wayland seat.
- Passes X11 windows through the same tiling layout as native windows.

## Limitations

- XWayland does not support GPU-accelerated rendering; X clients render via indirect software paths.
- Some X11 extensions (screen grab, certain sync tricks) are unsupported or emulated.
- Drag-and-drop between X11 and Wayland clients is limited by XWayland.
- Screen recording tools that rely on X11 shm may not capture composited output directly.
- Fractional scaling of X11 windows is approximated; text may appear soft.

## Clipboard Bridging

Wayland primary selection and clipboard are bridged to the X11 selection. The `selection.c` module handles these translations:

- X11 `CLIPBOARD` -> Wayland clipboard
- Wayland clipboard -> X11 `CLIPBOARD`
- X11 `PRIMARY` -> Wayland primary selection

This keeps copy/paste working across X11 and Wayland applications.

## Disabling XWayland

To disable, set `enabled = false` in the `[xwayland]` section. X11 applications will then need a separate X server or a compatibility layer such as XWayland's standalone mode.
