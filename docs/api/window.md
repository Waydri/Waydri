# WindowManager API

The `WindowManager` is the central authority for window lifecycle, focus, placement, and state management.

## Construction

```rust
let mut window_mgr = WindowManager::new();
```

Creates a WindowManager with no windows. Windows are added as clients connect and request surfaces.

## Methods

### `create_window(&mut self, props: WindowProperties) -> WindowId`

Creates a new window with the specified properties and returns its unique ID. The window is added to the active workspace and tiled according to the current layout.

```rust
let id = window_mgr.create_window(WindowProperties {
    title: "Terminal".to_string(),
    app_id: "org.alacritty".to_string(),
    min_size: Some((300, 200)),
    max_size: None,
    resizable: true,
});
```

### `close_window(&mut self, id: WindowId) -> Result<()>`

Closes the specified window. Triggers unmap events and redistributes remaining windows in the layout. Returns an error if the window does not exist.

### `focus_window(&mut self, id: WindowId) -> Result<()>`

Moves keyboard focus to the specified window. Updates the focused window state and sends activation events. Returns an error if the window does not exist.

### `set_placement(&mut self, id: WindowId, placement: Placement)`

Sets the placement mode for a window. Changes between tiled, floating, and fullscreen modes.

```rust
window_mgr.set_placement(id, Placement::Floating {
    x: 100,
    y: 100,
    width: 800,
    height: 600,
});
```

### `windows(&self) -> &[Window]`

Returns a slice of all managed windows.

### `focused_window(&self) -> Option<&Window>`

Returns a reference to the currently focused window.

### `workspace_windows(&self, workspace: usize) -> Vec<&Window>`

Returns all windows belonging to the specified workspace.

### `get_window(&self, id: WindowId) -> Option<&Window>`

Returns a reference to the window with the given ID.

## WindowState

```rust
pub enum WindowState {
    Normal,
    Floating,
    Fullscreen,
    Minimized,
    Maximized,
    Tiled,
}
```

### Normal

Default state. Window is tiled according to the current layout algorithm.

### Floating

Window is positioned absolutely and floats above tiled windows. Used for dialog boxes and tooltips.

### Fullscreen

Window fills the entire output area, covering all other windows and decorations.

### Minimized

Window is hidden from view but remains in the window list. Can be restored to its previous state.

### Maximized

Window expands to fill the available space between panel areas while retaining window decorations.

### Tiled

Explicit tiled state. Same as Normal but explicitly set by the user or IPC command.

## TilingTree

Each workspace maintains a `TilingTree` for computing tiled window positions. The WindowManager delegates layout calculations to the active layout through the TilingTree.

### Operations

- `insert`: Add a window to the tree at the current focus position.
- `remove`: Remove a window and rebalance the tree.
- `swap`: Exchange positions of two windows in the tree.
- `focus_next` / `focus_prev`: Move focus between windows.
- `split`: Change the split direction of a node.

## Window Properties

```rust
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub app_id: String,
    pub state: WindowState,
    pub geometry: Rect,
    pub workspace: usize,
    pub opacity: f32,
    pub decorated: bool,
    pub resizable: bool,
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
}
```

## Events

The WindowManager emits events that are forwarded through the IPC server:

- `window_created`: A new window was created.
- `window_destroyed`: A window was closed.
- `window_focus_changed`: Focus moved to a different window.
- `window_state_changed`: A window's state changed.
- `window_moved`: A window was moved to a different position.
- `window_resized`: A window was resized.

## Integration

```rust
let mut window_mgr = WindowManager::new();
let mut layout_mgr = LayoutManager::with_default();

for workspace in 0..10 {
    window_mgr.set_layout(workspace, "tile");
}

let id = window_mgr.create_window(WindowProperties {
    title: "My Window".to_string(),
    app_id: "com.example.app".to_string(),
    min_size: None,
    max_size: None,
    resizable: true,
});

window_mgr.focus_window(id)?;

let area = Rect::new(0, 0, 1920, 1080);
let rects = layout_mgr.arrange(
    window_mgr.workspace_layout(0),
    area,
    &window_mgr.workspace_window_ids(0),
);
```
