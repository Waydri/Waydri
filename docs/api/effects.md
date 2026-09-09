# Effects API

The `waydri_core::effects` module applies visual effects such as blur, opacity, and shadows.

## Types

### `EffectManager`

```rust
pub struct EffectManager {
    pub blur_enabled: bool,
    pub blur_size: u32,
    pub opacity_focused: f32,
    pub opacity_unfocused: f32,
    pub vibrance: f32,
    pub shadow_enabled: bool,
}
```

### `EffectState`

Per-window effect state.

```rust
pub struct EffectState {
    pub opacity: f32,
    pub blur: bool,
    pub rounded: bool,
    pub shadow: bool,
}
```

## Methods

| Method | Description |
|--------|-------------|
| `fn apply(&self, window: &Window, state: &mut EffectState)` | Apply global effects to a window |
| `fn set_blur(&mut self, enabled: bool, size: u32)` | Configure blur |
| `fn set_opacity(&mut self, focused: f32, unfocused: f32)` | Configure opacities |
| `fn render(&self, ctx: &RenderContext, region: &Region)` | Render effects for a region |

## Example

```rust
use waydri_core::effects::{EffectManager, EffectState};

let mut effects = EffectManager::default();
effects.set_blur(true, 8);
effects.set_opacity(1.0, 0.85);

let mut state = EffectState::default();
effects.apply(&window, &mut state);
```
