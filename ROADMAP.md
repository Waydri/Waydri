# Roadmap

Planned work for Waydri, ordered by target release. Items may be reordered
as priorities change.

## 0.12.0

- Wayland protocol server for client surfaces.
- Damage tracking integrated with the render loop.
- Shared memory buffer exchange between clients and the compositor.
- Config hot reload.

## 0.13.0

- Hardware acceleration for the blur, shadow, and gradient effects.
- Vulkan command buffer generation for the software renderer.
- Per-output rendering with mixed backends.

## 0.14.0

- Plugin loader for Lua plugins.
- Plugin sandboxing with capability isolation.
- IPC event subscriptions for external tools.

## 1.0.0

- Stable IPC protocol.
- Stable configuration schema.
- XWayland integration complete.
- Android native activity rendering with hardware acceleration.

## Longer term

- Multi-touch gesture engine for workspace and window control.
- Screen recording and screenshot capture through IPC.
- Remote rendering over the network.
- Session recovery after crash.
- Accessibility integration.