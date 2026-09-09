# wlr-layer-shell Protocol

## Overview

wlr-layer-shell is a Wayland protocol extension that allows applications to create surfaces at specific layers of the compositor. It is commonly used for desktop shells, taskbars, notification daemons, and wallpaper applications.

## Version

Waydri supports wlr-layer-shell version 4.

## Interfaces

### zwlr_layer_shell_v1

The factory interface for creating layer surfaces. Bound as a global singleton.

#### get_layer_surface

```
get_layer_surface(id: new_id zwlr_layer_surface_v1, surface: wl_surface, output: wl_output?, layer: uint32)
```

Creates a new layer surface on the specified layer. The output parameter is optional; if null, the surface is placed on all outputs.

### zwlr_layer_surface_v1

Manages a single layer surface.

#### set_size

```
set_size(width: uint32, height: uint32)
```

Sets the desired size of the surface. Use 0 for auto-sizing.

#### set_anchor

```
set_anchor(anchor: uint32)
```

Sets the anchor point for the surface. The surface is positioned relative to the anchor.

#### set_exclusive_zone

```
set_exclusive_zone(int32: zone)
```

Sets the exclusive zone. Windows are pushed away from this surface by the specified number of pixels.

#### set_keyboard_interactivity

```
set_keyboard_interactivity(mode: uint32)
```

Sets how the surface interacts with keyboard input.

#### get_popup

```
get_popup(id: new_id zwlr_layer_surface_v1, popup: xdg_popup)
```

Associates a popup with this layer surface.

#### ack_configure

```
ack_configure(serial: uint32)
```

Acknowledges a configure event from the compositor.

## Layers

| Layer | Value | Description |
|-------|-------|-------------|
| background | 0 | Below all other windows |
| bottom | 1 | Above background, below regular windows |
| top | 2 | Above regular windows, below overlay |
| overlay | 3 | Above all windows |

## Anchors

| Anchor | Value | Description |
|--------|-------|-------------|
| none | 0 | No anchor, surface is centered |
| top | 1 | Anchored to the top edge |
| bottom | 2 | Anchored to the bottom edge |
| left | 4 | Anchored to the left edge |
| right | 8 | Anchored to the right edge |

Anchors can be combined: top+left = 5, bottom+right = 10.

## Keyboard Interactivity

| Mode | Value | Description |
|------|-------|-------------|
| none | 0 | No keyboard events |
| exclusive | 1 | Grabs all keyboard events |
| on_demand | 2 | Receives events when focused |

## Exclusive Zones

Exclusive zones prevent other layer surfaces and tiled windows from overlapping. Use negative values to allow other surfaces to overlap.

A taskbar with exclusive_zone=30 reserves 30 pixels from the screen edge, pushing tiled windows inward.

## Configuration Events

The compositor sends configure events with width, height, and serial. The client must respond with ack_configure and submit a buffer of the requested size.

```
configure(serial: uint32, width: uint32, height: uint32)
```

## Example Usage

```rust
let layer_surface = layer_shell.get_layer_surface(
    surface,
    Some(output),
    Layer::Bottom,
);

layer_surface.set_anchor(Anchor::Bottom | Anchor::Left | Anchor::Right);
layer_surface.set_size(0, 30);
layer_surface.set_exclusive_zone(30);
layer_surface.set_keyboard_interactivity(KeyboardMode::None);
```
