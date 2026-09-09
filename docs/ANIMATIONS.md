# Waydri Animation System

Waydri animates window movement, resizing, focus transitions, and workspace changes using a spring-physics engine with configurable curves.

## Springs

Animations use spring physics. Values are configured per animation:

```toml
[animations]
enabled = true
default_spring = "smooth"

[animations.springs.smooth]
stiffness = 170.0
damping = 26.0
mass = 1.0

[animations.springs.bouncy]
stiffness = 300.0
damping = 12.0
mass = 1.0
```

| Parameter | Effect |
|-----------|--------|
| `stiffness` | Resistance to deflection; higher is faster |
| `damping` | Resistance to motion; higher settles sooner |
| `mass` | Inertia; higher overshoots more |

## Curves

For non-spring transitions, an easing function can be selected:

```toml
[animations]
easing = "ease_in_out_cubic"
duration = 150
```

Built-in easing functions:

- `linear`
- `ease_in`, `ease_out`, `ease_in_out`
- `ease_in_cubic`, `ease_out_cubic`, `ease_in_out_cubic`
- `ease_in_quad`, `ease_out_quad`, `ease_in_out_quad`
- `ease_in_sine`, `ease_out_sine`, `ease_in_out_sine`
- `ease_in_back`, `ease_out_back`, `ease_in_out_back`

## Transitions

Animations that can be customized:

| Animation | Description |
|-----------|-------------|
| `window_move` | Window position on layout/move |
| `window_resize` | Window size change |
| `focus` | Focus highlight/border transition |
| `workspace_change` | Workspace slide/fade |
| `open` / `close` | Window appear/dismiss |

```toml
[animations.transitions.window_move]
spring = "smooth"
duration = 150

[animations.transitions.workspace_change]
easing = "ease_in_out_cubic"
duration = 180
```

## Custom Animations

Plugins can drive custom animations through the animation manager API:

```rust
use waydri_core::animation::Animation;

let anim = Animation::spring(stiffness, damping, mass)
    .duration(Duration::from_millis(200))
    .target(final_value);
```

See [tools/docs/api.md#animation](../tools/docs/api.md) for the animation manager API.

## Disabling Animations

```toml
[animations]
enabled = false
```

Set `enabled = false` to disable all animations for a snappier feel on low-power devices.
