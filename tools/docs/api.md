# Waydri Core API Reference

This document describes the core Rust API of Waydri v0.11.0. The API is organized into modules under the `waydri-core` crate.

## Module Overview

| Module | Purpose |
|--------|---------|
| `waydri::state` | Global compositor state |
| `waydri::compositor` | Compositor state machine |
| `waydri::layout` | Window layout management |
| `waydri::window` | Window management |
| `waydri::input` | Input handling |
| `waydri::ipc` | Inter-process communication |
| `waydri::render` | Rendering backend abstraction |
| `waydri::config` | Configuration loading |
| `waydri::animation` | Animation and spring physics |
| `waydri::effects` | Visual effects |
| `waydri::protocol` | Wayland protocol handlers |

## WaydriState

The `WaydriState` type is the root of the compositor and holds all subsystem state.

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `display` | `Display` | The wayland-server display |
| `event_loop` | `EventLoop` | The poll event loop |
| `compositor` | `CompositorState` | The compositor state machine |
| `layout` | `LayoutManager` | Layout management |
| `windows` | `WindowManager` | Window registry |
| `input` | `InputManager` | Input device handling |
| `ipc` | `IpcServer` | The IPC server |
| `config` | `Config` | Loaded configuration |
| `animation` | `AnimationManager` | Animation state |
| `running` | `bool` | Whether the main loop is running |

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `fn new(config: Config) -> Result<Self>` | Create a new compositor state |
| `run` | `fn run(self) -> Result<()>` | Start the main loop |
| `shutdown` | `fn shutdown(&mut self)` | Stop the compositor cleanly |
| `focus_window` | `fn focus_window(&mut self, id: WindowId)` | Focus a window |
| `close_window` | `fn close_window(&mut self, id: WindowId)` | Close a window |
| `spawn` | `fn spawn(&mut self, cmd: &str) -> Result<()>` | Spawn a shell command |

## CompositorState

`CompositorState` implements the compositor state machine, tracking frames, damage, and output.

### Fields

- `outputs: Vec<Output>` - Active outputs
- `current_mode: Mode` - Current compositor mode
- `frame_time: Duration` - Last frame duration
- `damage_tracking: bool` - Whether damage tracking is enabled
- `scale: f64` - Global output scale

### Methods

- `fn add_output(&mut self, output: Output)`
- `fn remove_output(&mut self, id: OutputId)`
- `fn commit_frame(&mut self) -> Result<()>`
- `fn set_mode(&mut self, mode: Mode)`
- `fn output_layout(&self) -> &OutputLayout`

## LayoutManager

`LayoutManager` manages the tiling layout applied to windows.

### Types

- `enum LayoutType { MasterStack, Grid, Dwindle, Custom }`
- `struct LayoutNode { window: WindowId, rect: Rect, split: Option<Split> }`
- `struct Split { ratio: f64, axis: Axis }`
- `enum Axis { Horizontal, Vertical }`

### Methods

- `fn new() -> Self`
- `fn set_layout(&mut self, layout: LayoutType)`
- `fn arrange(&mut self, windows: &[WindowId], area: Rect) -> Vec<LayoutNode>`
- `fn set_ratio(&mut self, ratio: f64)`
- `fn next_window(&self, current: Option<WindowId>) -> Option<WindowId>`

## WindowManager

`WindowManager` tracks all managed windows.

### Types

- `struct Window { id: WindowId, title: String, app_id: String, geometry: Rect, floating: bool, minimized: bool }`
- `struct WindowId(pub u64)`

### Methods

- `fn register(&mut self, window: Window) -> WindowId`
- `fn unregister(&mut self, id: WindowId)`
- `fn get(&self, id: WindowId) -> Option<&Window>`
- `fn get_mut(&mut self, id: WindowId) -> Option<&mut Window>`
- `fn set_floating(&mut self, id: WindowId, floating: bool)`
- `fn focus(&mut self, id: WindowId)`
- `fn focused(&self) -> Option<WindowId>`

## InputManager

`InputManager` manages input devices and pointer/keyboard state.

### Methods

- `fn new(seat: Seat) -> Self`
- `fn handle_key(&mut self, keycode: u32, state: KeyState)`
- `fn handle_pointer_motion(&mut self, x: f64, y: f64)`
- `fn handle_button(&mut self, button: u32, state: ButtonState)`
- `fn handle_axis(&mut self, delta: f64)`
- `fn dispatch_keymap(&mut self)`

## IpcServer

`IpcServer` provides a Unix-socket IPC interface for external control.

### Types

- `enum IpcMessage { Reload, Focus(WindowId), Close(WindowId), Spawn(String), GetState, Quit }`
- `enum IpcResponse { Ok, State(serde_json::Value), Error(String) }`

### Methods

- `fn bind(path: &Path) -> Result<Self>`
- `fn handle_request(&mut self, msg: IpcMessage) -> IpcResponse`
- `fn request(&self, msg: IpcMessage) -> Result<IpcResponse>`

## Config

The `Config` type holds parsed configuration and is loaded at startup.

### Fields

- `general: GeneralConfig`
- `keybinds: HashMap<Keycombo, Action>`
- `layout: LayoutConfig`
- `theme: ThemeConfig`
- `rules: Vec<Rule>`
- `plugins: Vec<PluginConfig>`

## Example

```rust
use waydri::state::WaydriState;
use waydri::config::Config;

fn main() -> Result<()> {
    let config = Config::load("waydri.toml")?;
    let mut state = WaydriState::new(config)?;
    state.run()
}
```
