# Supported Wayland Protocols

## Core Protocols

| Protocol | Version | Description |
|----------|---------|-------------|
| wl_compositor | 4 | Core compositor interface |
| wl_shm | 1 | Shared memory buffers |
| wl_data_device | 3 | Clipboard and drag-and-drop |
| wl_output | 4 | Output/display management |
| wl_seat | 8 | Input device management |
| wl_subcompositor | 1 | Subsurface compositing |
| wl_surface | 5 | Surface management |

## XDG Protocols

| Protocol | Version | Feature Flag | Description |
|----------|---------|--------------|-------------|
| xdg_wm_base | 3 | `xdg_shell` | Window management |
| xdg_decoration | 1 | `xdg_decoration` | Server-side decorations |
| xdg_foreign | 2 | `xdg_foreign` | Cross-process surface sharing |

## Layer Shell

| Protocol | Version | Feature Flag | Description |
|----------|---------|--------------|-------------|
| zwlr_layer_shell | 4 | `layer_shell` | Desktop shell surfaces |

## Selection and Clipboard

| Protocol | Version | Feature Flag | Description |
|----------|---------|--------------|-------------|
| zwp_primary_selection | 1 | `primary_selection` | Primary selection (middle-click) |
| wl_data_device | 3 | `clipboard` | Standard clipboard |

## Pointer and Input

| Protocol | Version | Feature Flag | Description |
|----------|---------|--------------|-------------|
| zwp_relative_pointer | 1 | `relative_pointer` | Relative pointer motion |
| zwp_pointer_constraints | 1 | `pointer_constraints` | Pointer lock and confinement |
| zwp_keyboard_shortcuts_inhibit | 1 | `shortcuts_inhibit` | Keyboard shortcut inhibition |

## Presentation

| Protocol | Version | Feature Flag | Description |
|----------|---------|--------------|-------------|
| wp_presentation | 1 | `presentation` | Frame timing and presentation |

## Feature Flags

Feature flags are set in the compositor configuration:

```lua
return {
    wayland = {
        protocols = {
            xdg_shell = true,
            layer_shell = true,
            primary_selection = true,
            relative_pointer = true,
            pointer_constraints = true,
            shortcuts_inhibit = true,
            xdg_decoration = true,
            presentation = true,
        },
    },
}
```
