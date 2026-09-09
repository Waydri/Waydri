# Compositor API

The `waydri_core::compositor` module implements the compositor state machine and output management.

## Types

### `CompositorState`

```rust
pub struct CompositorState {
    pub outputs: Vec<Output>,
    pub mode: Mode,
    pub frame_time: Duration,
    pub damage_tracking: bool,
    pub scale: f64,
}
```

Methods:

| Method | Signature | Description |
|--------|-----------|-------------|
| `add_output` | `fn add_output(&mut self, output: Output)` | Register an output |
| `remove_output` | `fn remove_output(&mut self, id: OutputId)` | Remove an output |
| `commit_frame` | `fn commit_frame(&mut self) -> Result<()>` | Present the current frame |
| `set_mode` | `fn set_mode(&mut self, mode: Mode)` | Change the compositor mode |
| `output_layout` | `fn output_layout(&self) -> &OutputLayout` | Access the output layout |

### `Mode`

```rust
pub enum Mode {
    Initializing,
    Running,
    Suspended,
    ShuttingDown,
}
```

### `Output`

```rust
pub struct Output {
    pub id: OutputId,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
    pub scale: f64,
    pub transform: Transform,
}
```

## Damage Tracking

Methods for damage region management:

| Method | Description |
|--------|-------------|
| `fn mark_damaged(&mut self, region: Region)` | Mark a region for redraw |
| `fn damaged_region(&self) -> &Region` | Current accumulated damage |
| `fn clear_damage(&mut self)` | Reset the damage tracker |

## Example

```rust
use waydri_core::compositor::{CompositorState, Mode, Output};

let mut state = CompositorState::default();
state.add_output(Output::new("DP-1", 2560, 1440, 144));
state.set_mode(Mode::Running);
state.commit_frame()?;
```
