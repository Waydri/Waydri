# Credits

Waydri is built on the work of many individuals and open-source projects.

## Project Maintainers

- Waydri core team as listed in [MAINTAINERS](../MAINTAINERS) and [AUTHORS](../AUTHORS).

## Contributors

All contributors who have submitted code, reported bugs, improved documentation, or otherwise supported the project are acknowledged in the project history and the [AUTHORS](../AUTHORS) file.

## Upstream Projects and Inspiration

Waydri's design and implementation draw heavily on the following projects:

### wlroots

The architecture concepts for output management, seat handling, and the event-driven compositor model are informed by wlroots. Geometry and lifecycle conventions (zombie surface handling, damage tracking) follow its patterns.

### Sway

A tiling Wayland compositor built on wlroots. Sway informed the master-stack and grid layout concepts and the IPC message design.

### Hyprland

A dynamic tiling compositor known for its animation and effects system. Hyprland inspired Waydri's spring-physics animation approach and the focus on visual polish.

### Wayland

The Wayland protocol and its reference implementations form the core communication standard that Waydri implements.

### Other Dependencies

- The Wayland protocol C headers and library
- Mesa and the GBM/KMS/DRI stack
- The Vulkan SDK and its ecosystem
- The Zig language and standard library for the shader backend

## Acknowledgments

We thank the broader Wayland, Linux graphics, Android, and open-source communities for the tools, libraries, and knowledge that make Waydri possible.
