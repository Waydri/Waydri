# Changelog

All notable changes to Waydri are documented in this file.

## v0.11.0

### New Features

- Vulkan renderer backend with GLES fallback
- Android immersive mode with edge-swipe gestures
- IPC server over Unix socket for compositor control
- Plugin system with lifecycle hooks and capability sandbox
- Spring-physics animation system
- Configurable grids and dwindle layout modes
- XWayland integration with clipboard bridging

### Fixes

- Fixed a memory leak in subsurface destruction
- Fixed primary-selection clipboard drops when the source client closes
- Corrected xdg_shell configure/ack_configure ordering
- Improved focus handling for layer-shell keyboards-only surfaces

### Performance

- Damage-based region re-rendering
- VBlank-synced frame scheduling
- Dmabuf import with reference counting

## Known Issues

- XWayland fractional scaling is approximated and may appear soft
- Android sleep/wake can occasionally desync the compositor from vblank
- The `grid` layout does not yet support explicit per-window column sizing

## Previous Releases

### v0.10.0

- Initial tiling compositor on X11/headless platforms
- Basic master_stack layout
- TOML configuration

### v0.9.0

- Prototype Wayland compositor skeleton
- Wayland protocol bindings for core interfaces
