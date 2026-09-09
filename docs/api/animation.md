# Animation API

The `waydri_core::animation` module implements the spring and curve animation engine.

## Types

### `Animation`

A single animatable value over time.

```rust
pub struct Animation {
    value: f64,
    target: f64,
    spring: Option<Spring>,
    easing: Easing,
    duration: Duration,
    start_time: Instant,
    running: bool,
}
```

Methods:

| Method | Signature | Description |
|--------|-----------|-------------|
| `spring` | `fn spring(stiffness: f64, damping: f64, mass: f64) -> Self` | Create a spring animation |
| `curve` | `fn curve(easing: Easing, duration: Duration) -> Self` | Create a curve animation |
| `target` | `fn target(mut self, t: f64) -> Self` | Set the target value |
| `start` | `fn start(&mut self)` | Begin the animation |
| `update` | `fn update(&mut self, now: Instant) -> f64` | Advance and return current value |
| `is_finished` | `fn is_finished(&self) -> bool` | Whether the animation settled |

### `Spring`

```rust
pub struct Spring {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
}
```

### `Easing`

```rust
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic(f64, f64, f64, f64),
}
```

### `AnimationManager`

Tracks all active animations and drives them each frame.

```rust
pub struct AnimationManager {
    animations: Vec<Animation>,
    max_animations: usize,
}
```

Methods:

| Method | Description |
|--------|-------------|
| `fn new() -> Self` | Create an empty manager |
| `fn add(&mut self, anim: Animation)` | Register an animation |
| `fn remove(&mut self, id: usize)` | Remove by index |
| `fn update_all(&mut self) -> Vec<f64>` | Advance all and return values |
| `fn clear(&mut self)` | Stop all animations |

## Example

```rust
use waydri_core::animation::{Animation, Easing};
use std::time::{Duration, Instant};

let mut anim = Animation::curve(Easing::EaseInOut, Duration::from_millis(150))
    .target(1.0);
anim.start();

let now = Instant::now();
loop {
    let value = anim.update(now);
    if anim.is_finished() { break; }
}
```
