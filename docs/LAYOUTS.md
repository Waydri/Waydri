# Waydri Layouts

Waydri provides multiple tiling layouts. Layouts are selected in the configuration file and can be cycled at runtime.

## Built-In Layouts

### master_stack

A main master window on one side with the remaining windows stacked beside it.

```toml
[layout]
primary = "master_stack"
master_size = 1
master_ratio = 0.55
```

The master area takes `master_ratio` of the screen width. `master_size` controls how many windows are tiled in the master column.

### grid

Windows are laid out in a roughly square grid.

```toml
[layout]
primary = "grid"
columns = 0
```

`columns` defaults to auto-computed from the window count.

### dwindle

A binary-splitting layout where each new window splits the largest free area.

```toml
[layout]
primary = "dwindle"
split_direction = "auto"
```

`split_direction` is `auto`, `horizontal`, or `vertical`.

### custom

User-defined layouts loaded from `config/layouts/*.lua` or built as plugins.

```toml
[layout]
primary = "custom"
custom = "my_layout"
```

## Layout Ratios

The tiling ratio (master ratio, dwindle split ratio) is adjustable at runtime:

| Key Combination | Action |
|-----------------|--------|
| `mod+[` | Decrease ratio |
| `mod+]` | Increase ratio |

Ratios are stored per-workspace and persist across layout switches.

## Layout API

Layouts implement the `Layout` trait:

```rust
pub trait Layout {
    fn name(&self) -> &str;
    fn arrange(&self, windows: &[WindowId], area: Rect, config: &LayoutConfig) -> Vec<LayoutNode>;
}
```

`LayoutNode` carries a window id and an adjusted rectangle:

```rust
pub struct LayoutNode {
    pub window: WindowId,
    pub rect: Rect,
    pub split: Option<Split>,
}
```

Custom layouts can be written in Rust as plugins or in Lua using the layout scripting API (see `tools/docs/api.md`).

## Arrangement Semantics

- Windows are arranged in focus order (most-recently-focused first).
- Floating windows are excluded from the tiling arrangement.
- Workspaces maintain an independent layout instance.
- Gaps and borders are subtracted from the output area before layout.

## Switching Layouts

Cycle through layouts:

```
waydri-ipc layout.cycle
```

Set a specific layout:

```
waydri-ipc layout.set grid
```

List available layouts:

```
waydri-ipc layout.list
```
