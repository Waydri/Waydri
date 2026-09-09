# Supported Wayland Protocols

Waydri implements the following Wayland protocol interfaces. Protocol XML stubs are documented under `docs/protocols/`.

## Core Interfaces

| Interface | Version | Notes |
|-----------|---------|-------|
| `wl_compositor` | 4 | Surface and subsurface management |
| `wl_shm` | 1 | Shared-memory buffers |
| `wl_seat` | 7 | Keyboard, pointer, touch |
| `wl_output` | 4 | Output mode/scale/transform |
| `wl_data_device_manager` | 3 | Clipboard and drag-and-drop |
| `xdg_shell` | 6 | Toplevel and popup surfaces |

## Extension Interfaces

| Interface | Version | Notes |
|-----------|---------|-------|
| `zwlr_layer_shell_v1` | 4 | Layer surfaces for bars/overlays |
| `zwp_relative_pointer_manager_v1` | 1 | Relative pointer motion |
| `wp_primary_selection` (`zwp_primary_selection_source_v1`) | 1 | Primary selection |
| `wlr_output_management_v1` | 1 | Dynamic output configuration |
| `xdg_output` | 3 | Output names and geometry |
| `zwp_pointer_gestures_v1` | 1 | Pinch/swipe gestures |
| `wp_viewporter` | 1 | Surface scaling and cropping |
| `wp_presentation` | 1 | Presentation timing feedback |

## Detailed Notes

### xdg_shell

Supports toplevel states: maximized, fullscreen, minimized, and tiled. Window configuration uses the standard configure/ack_configure cycle.

Protocol XML: [docs/protocols/xdg-shell.md](protocols/xdg-shell.md)

### wlr-layer-shell

Layer surfaces support all anchors, keyboard interactivity, and exclusive zones. Used by status bars such as waybar.

Protocol XML: [docs/protocols/wlr-layer-shell.md](protocols/wlr-layer-shell.md)

### zwp-relative-pointer

Provides deltas relative to pointer position. Pairs with the cursor lock/compositor-confine for games.

Protocol XML: [docs/protocols/zwp-relative-pointer.md](protocols/zwp-relative-pointer.md)

### wp-primary-selection

Primary selection clipboard with middle-click paste semantics across Wayland and XWayland.

Protocol XML: [docs/protocols/wp-primary-selection.md](protocols/wp-primary-selection.md)

## Versioning

Interface versions are negotiated per-client at bind time. The compositor advertises the highest version it implements; clients can bind at a lower version, and newer features are disabled accordingly.

## Running Protocol Compliance Tests

The protocol compliance suite verifies interface behavior:

```
cargo test --release --test protocol_compliance
```
