# LayoutManager API

The `LayoutManager` manages window tiling arrangements and provides the interface for registering and querying available layouts.

## Construction

```rust
let layout_mgr = LayoutManager::with_default();
```

Creates a LayoutManager with the default set of built-in layouts: `tile`, `monocle`, `dwindle`, `grid`, `center_master`.

## Methods

### `arrange(&self, name: &str, area: Rect, window_ids: &[WindowId]) -> Vec<Rect>`

Computes the position and size for each window using the named layout algorithm. Returns a vector of rectangles in the same order as the input window IDs.

```rust
let rects = layout_mgr.arrange(
    "dwindle",
    Rect::new(0, 0, 1920, 1080),
    &[win1, win2, win3],
);
```

### `available(&self) -> &[&str]`

Returns a slice of all registered layout names.

```rust
for name in layout_mgr.available() {
    println!("Layout: {}", name);
}
```

### `register(&mut self, name: &str, layout: Box<dyn Layout>)`

Registers a custom layout with the given name. If the name already exists, the previous layout is replaced.

```rust
layout_mgr.register("my_layout", Box::new(MyCustomLayout::new()));
```

### `get(&self, name: &str) -> Option<&dyn Layout>`

Returns a reference to the layout registered under the given name.

### `set_gaps(&mut self, gaps: GapConfig)`

Updates the gap configuration applied to all layouts. Gaps add spacing between windows and around screen edges.

```rust
layout_mgr.set_gaps(GapConfig {
    inner: 4,
    outer: 8,
    top: 0,
    bottom: 0,
    left: 0,
    right: 0,
});
```

## TilingTree

The `TilingTree` is a binary tree data structure used internally by tiling layouts to compute window positions.

### Methods

#### `insert_leaf(&mut self, window_id: WindowId)`

Adds a window as a new leaf in the tree. The insertion position follows the layout's split policy.

#### `remove(&mut self, window_id: WindowId)`

Removes a window from the tree. If the tree becomes unbalanced, it is rebalanced automatically.

#### `arrange(&self, area: Rect, gaps: &GapConfig) -> HashMap<WindowId, Rect>`

Traverses the tree and computes the final rectangle for each window. Splits are distributed according to the tree structure and gap settings.

#### `focus_next(&mut self) -> Option<WindowId>`

Moves focus to the next window in the tree order.

#### `focus_prev(&mut self) -> Option<WindowId>`

Moves focus to the previous window in the tree order.

#### `swap_focused(&mut self, direction: SwapDirection)`

Swaps the focused window with its neighbor in the specified direction (Up, Down, Left, Right).

## SplitAxis

```rust
pub enum SplitAxis {
    Horizontal,
    Vertical,
}
```

Defines the direction in which a tree node is split. Horizontal splits place windows side by side; vertical splits stack them.

## SplitPos

```rust
pub enum SplitPos {
    Left,
    Right,
    Top,
    Bottom,
    First,
    Last,
}
```

Determines the position of a newly inserted window relative to the focused node.

## Built-in Layouts

### tile

Standard master-stack layout. The first window gets the master area on the left; remaining windows stack on the right.

### monocle

All windows fill the entire screen, stacked on top of each other. Only the focused window is visible.

### dwindle

Binary split layout that divides the screen into halves. New windows are placed in the smaller half.

### grid

Places windows in a grid pattern, distributing them evenly across the available area.

### center_master

Centers the master window with a fixed ratio and places stack windows on either side.

## Custom Layouts

Implement the `Layout` trait to create custom layouts:

```rust
pub trait Layout: Send + Sync {
    fn name(&self) -> &str;
    fn arrange(&self, area: Rect, window_ids: &[WindowId], gaps: &GapConfig) -> Vec<Rect>;
    fn cycle_focus(&self, direction: CycleDirection) -> Option<usize>;
}
```
