# WorkspaceManager API

The `WorkspaceManager` handles virtual desktops, allowing users to organize windows across multiple workspaces.

## Construction

```rust
let workspace_mgr = WorkspaceManager::new(10);
```

Creates a WorkspaceManager with 10 virtual workspaces (0-9). The first workspace is active by default.

## Methods

### `active_workspace(&self) -> usize`

Returns the index of the currently active workspace.

### `switch_to(&mut self, index: usize) -> Result<()>`

Switches to the specified workspace. Returns an error if the index is out of bounds. When switching, the previous workspace's windows are hidden and the new workspace's windows are shown.

```rust
workspace_mgr.switch_to(3)?;
```

### `workspace(&self, index: usize) -> Option<&Workspace>`

Returns a reference to the workspace at the given index.

### `next_workspace(&mut self)`

Cycles to the next workspace. Wraps from the last workspace to the first.

### `prev_workspace(&mut self)`

Cycles to the previous workspace. Wraps from the first workspace to the last.

### `move_to_workspace(&mut self, window_id: WindowId, workspace: usize) -> Result<()>`

Moves a window to a different workspace. The window is removed from the current workspace and added to the target workspace.

### `visible_workspaces(&self) -> Vec<usize>`

Returns the indices of all visible workspaces. On multi-monitor setups, each output can show a different workspace.

### `show_workspace(&mut self, index: usize, output: OutputId)`

Makes the specified workspace visible on the given output.

### `set_layout(&mut self, workspace: usize, layout: &str) -> Result<()>`

Sets the layout for a specific workspace. Different workspaces can use different layouts.

### `workspace_count(&self) -> usize`

Returns the total number of workspaces.

## Workspace

```rust
pub struct Workspace {
    pub index: usize,
    pub name: String,
    pub visible: bool,
    pub layout_name: String,
    pub windows: Vec<WindowId>,
    pub tiling_tree: TilingTree,
}
```

### `layout_name(&self) -> &str`

Returns the name of the layout currently active on this workspace.

### `window_ids(&self) -> &[WindowId]`

Returns the IDs of all windows belonging to this workspace.

### `toggle_visibility(&mut self)`

Toggles the workspace's visibility state. Hidden workspaces are not rendered but retain their window list.

### `add_window(&mut self, window_id: WindowId)`

Adds a window to the workspace. Called automatically by WindowManager when a window is created or moved.

### `remove_window(&mut self, window_id: WindowId)`

Removes a window from the workspace. Called automatically when a window is closed or moved.

### `focused_window(&self) -> Option<WindowId>`

Returns the ID of the focused window in this workspace.

### `set_focused(&mut self, window_id: WindowId)`

Sets the focused window for this workspace.

### `arrange(&self, area: Rect, gaps: &GapConfig) -> HashMap<WindowId, Rect>`

Computes the layout positions for all windows in this workspace using the current layout.

### Workspace Names

Workspaces can be assigned custom names in configuration:

```lua
return {
    workspaces = {
        names = { "code", "browser", "terminal", "media", "chat" },
    },
}
```

## Multi-Monitor

In multi-monitor setups, each output can show a different workspace. The WorkspaceManager tracks which workspaces are visible on which outputs.

```rust
for output in output_mgr.outputs() {
    let ws = workspace_mgr.active_for_output(output.id);
    println!("Output {} shows workspace {}", output.name, ws);
}
```

## Sticky Windows

Windows can be marked as sticky, appearing on all workspaces:

```rust
workspace_mgr.set_sticky(window_id, true);
```

Sticky windows are excluded from workspace switching and remain visible across all virtual desktops.
