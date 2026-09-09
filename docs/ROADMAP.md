# Waydri Roadmap

This document outlines the planned direction for Waydri. Timeline milestones are approximate and subject to change based on contributor availability.

## v0.12 - Multi-GPU

- Support for multi-GPU rendering with per-output device selection
- Explicit sync and multi-plane overlay support
- Output hotplug improvements
- Per-monitor layout configuration
- Wayland color-management protocol draft support

Target: next minor release.

## v0.13 - Virtual Reality

- Head-tracked rendering through the Vulkan VR extension
- Side-by-side stereoscopic output for existing windows in VR
- Input remapping (hand controllers to pointer/keyboard)
- A compositor shell optimized for immersive workspaces
- Experimental support for a single supported VR headset

Target: after v0.12.

## v1.0 - Stable API

- Freeze the plugin ABI and IPC protocol (semantic versioning)
- Guarantee backward compatibility for configuration files
- Complete Wayland protocol surface for a production compositor
- Bring up end-to-end on a reference Android device and a reference Linux laptop
- Establish a stable release process and maintenance policy

Target: after v0.13 stabilizes the core feature set.

## Ongoing Priorities

- Rendering performance and frame scheduling
- Security hardening of the plugin sandbox and IPC
- Documentation completeness
- Test and CI coverage expansion
