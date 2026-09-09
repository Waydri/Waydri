# Waydri Wayland Protocol Support

This document lists the Wayland protocol interfaces implemented by Waydri v0.11.0, with their versions and feature notes.

## Core Protocol

### wl_compositor (version 4)

The base compositor interface. Waydri supports surfaces, subsurfaces, and region handling. Requires clients to support at least version 3 for proper buffer damage handling. Version 4 adds support for the `wl_surface.damage_buffer` semantics.

### wl_shm (version 1)

Shared memory buffer pool management. Supports ARGB8888, XRGB8888, and the standard pixel formats. Used by software-rendered clients and as feedback fallback.

### wl_seat (version 7)

Seat management. Supports the full keyboard, pointer, and touch interfaces. Pointer focus follows cursor movement; keyboard focus follows window focus.

### wl_output (version 4)

Output device properties. Waydri exposes one output per connected display, with mode, scale, and transform information. On Android, the output maps to the device framebuffer.

### xdg_shell (version 6)

The desktop-shell extension for regular application windows. Supports toplevel and popup surfaces, window states (maximized, fullscreen, minimized), and configuration cycles. Waydri requires xdg_shell version 4 minimum.

### wl_data_device_manager (version 3)

Clipboard and drag-and-drop data exchange. Supports multiple mime types, offer/select semantics, and source data insertion.

## Unstable and Extension Protocols

### wlr-layer-shell (version 1)

Layer surfaces for panels, bars, and overlays. Used by status bars such as waybar. Supports anchors, keyboard interactivity, and exclusive zones.

### zwp_relative_pointer_manager_v1 (version 1)

Relative pointer motion events, useful for games and mouse-capture applications. Pairs with the absolute-pointer manager to switch between absolute and relative motion.

### wp_primary_selection (version 1)

Primary selection clipboard. Provides middle-click paste semantics for X11-compatible workflows.

### wlr_output_management (version 1)

Output configuration protocol. Waydri uses this to apply monitor layout, resolution, and rotation changes requested by clients.

### zwlr_layer_shell_v1 (version 4)

The compositor-side implementation of layer surfaces, exposing the version 4 interface.

### xdg_output (version 3)

Exposes output names and logical geometry. Important for fractional scaling support.

### zwp_pointer_gestures_v1 (version 1)

Pointer gesture events for pinch and swipe, used by touchpads and trackpads.

## Protocol Versioning

Waydri negotiates protocol versions with clients at bind time. The compositor advertises the highest version it implements, allowing older clients to bind at a lower interface version. Features gated behind newer versions are disabled when a client binds at an older version.

The `WAYDRI_PROTOCOL_VERSION` constant reports the protocol implementation level. Individual interfaces are negotiated per-client and do not require globals re-advertisement.

## Implementation Notes

- All core interfaces are registered as global Wayland objects.
- The `zombie` resource list is tracked to defer destroys until the frame is committed, matching the wlroots event ordering model.
- Non-mappable surfaces are freed when their corresponding client disconnects.
- The damage tracking system reports only damaged regions to the renderer to minimize redraw cost.
