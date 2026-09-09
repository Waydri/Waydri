# zwp_relative_pointer Protocol

## Overview

zwp_relative-pointer provides relative pointer motion events. This is essential for applications that need raw mouse movement data, such as 3D viewers, games, and pointer-lock scenarios.

## Version

Waydri supports zwp_relative-pointer version 1.

## Interfaces

### zwp_relative_pointer_manager_v1

Factory for creating relative pointer instances. Bound as a global singleton.

#### get_relative_pointer

```
get_relative_pointer(id: new_id zwp_relative_pointer, pointer: wl_pointer)
```

Creates a relative pointer associated with the specified wl_pointer.

### zwp_relative_pointer_v1

Receives relative motion events for a pointer.

#### relative_motion

```
relative_motion(utime_hi: uint32, utime_lo: uint32, dx: fixed, dy: fixed, dx_unaccelerated: fixed, dy_unaccelerated: fixed)
```

Event: pointer moved relative to its previous position. dx/dy are accelerated values; dx_unaccelerated/dy_unaccelerated are raw values without acceleration.

## Relative Motion Events

```rust
pub struct RelativeMotionEvent {
    pub time: u64,
    pub dx: f64,
    pub dy: f64,
    pub dx_unaccelerated: f64,
    pub dy_unaccelerated: f64,
}
```

### Accelerated vs Unaccelerated

- **Accelerated**: Subject to the system's pointer acceleration curve. Suitable for cursor movement.
- **Unaccelerated**: Raw hardware data. Suitable for FPS games and 3D applications.

## Pointer Lock

Pointer lock hides the cursor and provides continuous relative motion events. The cursor position is frozen, and all mouse movement is reported as relative motion.

### Lock Flow

1. Client acquires pointer lock on its surface.
2. Compositor hides the cursor and centers it.
3. Client receives only relative_motion events.
4. Client releases the lock to restore normal behavior.

## Confinement

Confinement restricts the pointer to stay within a defined region. Movement outside the region is clamped to the boundary.

### Confine Flow

1. Client requests pointer confinement to its surface.
2. Compositor restricts pointer movement to the surface bounds.
3. Pointer motion events are clamped at the surface edges.
4. Client releases confinement to restore normal behavior.

## Integration

Waydri manages relative pointer state per seat. When a client acquires pointer lock or confinement:

1. The compositor stores the lock/confinement state.
2. Normal pointer focus rules are bypassed.
3. Relative motion events are dispatched only to the locking client.
4. On unlock, normal focus rules resume.

## Use Cases

- FPS games: Lock pointer for mouse-look controls.
- 3D CAD: Free rotation without cursor leaving the viewport.
- Remote desktop: Relative motion for accurate cursor forwarding.
- VR/AR: Head tracking and pointer control.
