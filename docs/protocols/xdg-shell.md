# xdg-shell Protocol

## Overview

xdg-shell is the core Wayland protocol for window management. It provides interfaces for creating toplevel windows, popups, and configuring window state.

## Version

Waydri supports xdg-shell version 3.

## Interfaces

### xdg_wm_base

The top-level interface for creating xdg surfaces. Bound as a global singleton.

#### get_xdg_surface

```
get_xdg_surface(id: new_id xdg_surface, surface: wl_surface)
```

Creates an xdg_surface from a wl_surface.

#### pong

```
pong(serial: uint32)
```

Responds to a ping event. Clients that fail to pong within 60 seconds are considered unresponsive.

### xdg_surface

Base interface for xdg surface operations.

#### get_toplevel

```
get_toplevel(id: new_id xdg_toplevel)
```

Assigns the toplevel role to the surface.

#### get_popup

```
get_popup(id: new_id xdg_popup, parent: xdg_surface?, positioner: xdg_positioner)
```

Assigns the popup role to the surface.

#### ack_configure

```
ack_configure(serial: uint32)
```

Acknowledges a configure event from the compositor.

### xdg_toplevel

Manages a toplevel window's state and properties.

#### set_title

```
set_title(title: string)
```

Sets the window title.

#### set_app_id

```
set_app_id(app_id: string)
```

Sets the application identifier.

#### set_parent

```
set_parent(parent: xdg_toplevel)
```

Sets the parent window for modal behavior.

#### set_min_size

```
set_min_size(width: int32, height: int32)
```

Sets the minimum window size.

#### set_max_size

```
set_max_size(width: int32, height: int32)
```

Sets the maximum window size.

#### set_maximized

```
set_maximized()
```

Requests the window to be maximized.

#### unset_maximized

```
unset_maximized()
```

Requests the window to exit maximized state.

#### set_fullscreen

```
set_fullscreen(output: wl_output)
```

Requests the window to be fullscreen on the specified output.

#### unset_fullscreen

```
unset_fullscreen()
```

Requests the window to exit fullscreen state.

#### set_minimized

```
set_minimized()
```

Requests the window to be minimized.

#### set_resizable

```
set_resizable(resizable: boolean)
```

Sets whether the window can be resized by the user.

#### set_moved

```
set_moved()
```

Acknowledges a move request from the compositor.

#### set_resizing

```
set_resizing(resizing: boolean)
```

Indicates the client is resizing in response to a compositor request.

### xdg_positioner

Configures the position and behavior of popups.

#### set_size

```
set_size(width: int32, height: int32)
```

Sets the popup size.

#### set_anchor_rect

```
set_anchor_rect(x: int32, y: int32, width: int32, height: int32)
```

Sets the anchor rectangle relative to the parent surface.

#### set_anchor

```
set_anchor(anchor: uint32)
```

Sets the anchor point for positioning.

#### set_gravity

```
set_gravity(gravity: uint32)
```

Sets the gravity for positioning.

#### set_constraint_adjustment

```
set_constraint_adjustment(constraint_adjustment: uint32)
```

Sets how the popup adjusts when it would overflow the screen.

#### set_offset

```
set_offset(x: int32, y: int32)
```

Sets an offset from the calculated position.

## Toplevel States

| State | Value | Description |
|-------|-------|-------------|
| maximized | 1 | Window is maximized |
| fullscreen | 2 | Window is fullscreen |
| resizing | 3 | Window is being resized |
| activated | 4 | Window is focused/active |

## Configure Flow

1. Compositor sends configure with width, height, and states.
2. Client attaches a buffer of the new size.
3. Client sends ack_configure with the serial.
4. Compositor applies the configuration.

## Ping/Pong

The compositor periodically pings clients to detect unresponsiveness:

1. Compositor sends ping with a serial.
2. Client must respond with pong.
3. If no pong within 60 seconds, the client is marked unresponsive.
