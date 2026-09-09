# OutputManager API

The `OutputManager` tracks connected displays and manages their configuration, modes, and composition.

## Construction

```rust
let output_mgr = OutputManager::new();
```

Creates a new OutputManager with no outputs. Outputs are added dynamically as displays are connected or through manual headless registration.

## Methods

### `add_headless(&mut self, name: &str, width: u32, height: u32, fps: u32) -> OutputId`

Adds a headless (virtual) output with the specified dimensions and refresh rate. Useful for testing and off-screen rendering.

```rust
let headless = output_mgr.add_headless("virtual-1", 1920, 1080, 60);
```

### `remove_output(&mut self, id: OutputId)`

Removes an output and redistributes its windows to adjacent outputs.

### `focused_geometry(&self) -> Rect`

Returns the geometry (position and size) of the currently focused output. Falls back to the first available output if none is focused.

### `total_area(&self) -> Rect`

Returns the bounding rectangle covering all connected outputs. For multi-monitor setups, this is the union of all output geometries.

### `compose(&self) -> FrameBuffer`

Composites all output layers into the final frame buffer. Each output is composited independently and composed into a single frame for display.

```rust
let frame = output_mgr.compose();
display.present(&frame);
```

### `add_mode(&mut self, id: OutputId, mode: OutputMode)`

Adds a supported display mode to an output. Modes define resolution and refresh rate combinations.

```rust
output_mgr.add_mode(output_id, OutputMode {
    width: 2560,
    height: 1440,
    refresh_rate: 144,
});
```

### `remove_mode(&mut self, id: OutputId, mode: OutputMode)`

Removes a display mode from an output. Cannot remove the currently active mode.

### `set_mode(&mut self, id: OutputId, mode: OutputMode) -> Result<()>`

Switches an output to the specified mode. Returns an error if the mode is not supported.

### `outputs(&self) -> &[Output]`

Returns a slice of all currently connected outputs.

### `focused(&self) -> Option<OutputId>`

Returns the ID of the currently focused output, or `None` if no output is connected.

### `focus_output(&mut self, id: OutputId)`

Sets the specified output as focused. Input events are routed to windows on the focused output.

## Output

```rust
pub struct Output {
    pub id: OutputId,
    pub name: String,
    pub geometry: Rect,
    pub modes: Vec<OutputMode>,
    pub active_mode: OutputMode,
    pub scale: u32,
    pub transform: Transform,
    pub physical_size: (u32, u32),
}
```

### OutputMode

```rust
pub struct OutputMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}
```

### Transform

```rust
pub enum Transform {
    Normal,
    Rotated90,
    Rotated180,
    Rotated270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}
```

## Multi-Monitor

In multi-monitor setups, outputs are positioned side by side by default. Windows can be moved between outputs using IPC commands. Each output maintains its own workspace and focus state.

```rust
let total = output_mgr.total_area();
for output in output_mgr.outputs() {
    let local = output.geometry.translate(-total.x, -total.y);
    layout_mgr.arrange("tile", local, &workspace.window_ids());
}
```

## Events

The OutputManager emits events when outputs are added or removed. These are forwarded through the IPC server as `output_added` and `output_removed` events.
