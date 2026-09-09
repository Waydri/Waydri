# Waydri Architecture

Waydri is a Wayland compositor written primarily in Rust, with a Zig backend and a C XWayland compatibility layer. This document describes its system architecture.

## High-Level Layout

```
+--------------------------------------------------------------+
| waydri (binary)                                              |
|   entry point, startup, signal handling                      |
+--------------------------------------------------------------+
| waydri-core (workspace crate)                                |
|   WaydriState                                                |
+--------------------------------------------------------------+
| compositor | layout | window | input | ipc | config | anim  |
+--------------------------------------------------------------+
| renderer: Vulkan / GLES                                       |
+--------------------------------------------------------------+
| platform backend: Linux DRM/KMS | Android surface  | X11     |
+--------------------------------------------------------------+
```

## Core Modules

### Compositor State Machine

`CompositorState` drives the compositing loop. It tracks outputs, the current mode, frame timing, and damage regions. Each event-loop iteration:

1. Polls the display for client events.
2. Collects new surface damage.
3. Runs the state machine transitions.
4. Arranges windows via the layout manager.
5. Renders the damaged region.
6. Presents the frame.

### Renderer

The renderer abstracts Vulkan and GLES. It imports client buffers (shm and dmabuf), composites them with damage tracking, and applies blur and other effects. The Zig backend compiles shaders at build time.

### Layout Manager

`LayoutManager` computes window rectangles from the tiling layout and the output area. See [LAYOUTS.md](LAYOUTS.md).

### Window Manager

`WindowManager` tracks all windows, their geometry, floating state, and focus. Windows are identified by `WindowId`.

### Input Manager

`InputManager` handles the Wayland seat: keyboard, pointer, and touch devices. It dispatches keymaps, pointer motion, and gestures, converting them to focus and layout actions.

### IPC Server

`IpcServer` exposes a Unix-socket interface for external control (spawn, focus, theme, layout, plugin, and state queries).

### Wayland Protocol Handling

The `protocol` module implements Wayland server bindings for core and extension interfaces. See [PROTOCOLS.md](PROTOCOLS.md).

## State Machine

The compositor runs through the following states:

- `Initializing` - startup, output discovery
- `Running` - normal compositing
- `Suspended` - display power management
- `ShuttingDown` - graceful teardown

Transitions are triggered by signals (SIGTERM, SIGINT), IPC `Quit`, or input.

## IPC

Messages are serialized as JSON over a Unix socket (default `WAYDRI_SOCKET` or `$XDG_RUNTIME_DIR/waydri.sock`). The reference schema is in `docs/reference/ipc.json`.

## Wayland Protocol Handling

Wayland globals are registered at startup. Interface versions are negotiated per-client at bind time. Client resources that cannot be mapped are freed on disconnect to avoid leaks.

## Further Reading

- [docs/design/architecture.md](design/architecture.md)
- [docs/design/rendering-pipeline.md](design/rendering-pipeline.md) (see design docs)
- [tools/docs/api.md](../tools/docs/api.md)
