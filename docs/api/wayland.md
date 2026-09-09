# Wayland Protocol API

Waydri implements a subset of Wayland protocols for compositor-client communication. The protocol layer handles wire-format parsing, object lifecycle, and event dispatching.

## protocol::Protocol

The `Protocol` struct represents a Wayland protocol definition including its XML interface, version, and opcode table.

```rust
pub struct Protocol {
    pub name: String,
    pub version: u32,
    pub opcodes: Vec<Opcodes>,
}

pub struct Opcodes {
    pub interface: String,
    pub opcode: u16,
    pub name: String,
    pub since: u32,
    pub request: bool,
}
```

### Methods

#### `from_xml(xml: &str) -> Result<Protocol>`

Parses a Wayland protocol XML file and extracts all interfaces, requests, and events with their opcodes.

#### `opcode(&self, interface: &str, name: &str) -> Option<u16>`

Returns the numeric opcode for a given interface method. Used for wire-format encoding.

#### `interface(&self, name: &str) -> Option<&InterfaceDef>`

Returns the definition for a named interface including its requests, events, and enum types.

## wlr-layer-shell

The `wlr_layer_shell` module implements the wlr-layer-shell protocol, which allows applications to create surfaces at specific layers of the compositor.

### Supported Interfaces

- `zwlr_layer_shell_v1`: Creates layer surfaces. Version 4.
- `zwlr_layer_surface_v1`: Manages a single layer surface with anchor and exclusive zone.

### Layer Ordering

- `background`: Below all other windows (wallpaper, desktop widgets).
- `bottom`: Above background, below regular windows (taskbar area).
- `top`: Above regular windows, below overlay (status bars).
- `overlay`: Above all windows (notifications, popups).

### Anchor

```rust
pub enum Anchor {
    None,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
```

### Keyboard Interactivity

- `none`: Surface does not receive keyboard events.
- `exclusive`: Surface grabs all keyboard events within its area.
- `on_demand`: Surface receives keyboard events only when focused.

## xdg-shell

The `xdg_shell` module implements the xdg-shell protocol for standard window management.

### Supported Interfaces

- `xdg_wm_base`: Factory for creating xdg surfaces. Version 3.
- `xdg_surface`: Base surface with role assignment and configure/ack_configure.
- `xdg_toplevel`: Toplevel window with title, app_id, state, and geometry.
- `xdg_popup`: Popup surface with parent reference and positioning.

### Toplevel States

- `maximized`: Window is maximized to fill available space.
- `fullscreen`: Window fills the entire output.
- `resizing`: Window is being resized by the user.
- `activated`: Window is the currently active window.

### Configure Flow

1. Compositor sends `xdg_toplevel.configure` with width, height, and state.
2. Client attaches a new buffer and sends `xdg_surface.ack_configure` with the serial.
3. Compositor applies the new configuration.

## wp-primary-selection

The `wp_primary_selection` module implements the primary selection protocol for middle-click paste functionality.

### Supported Interfaces

- `zwp_primary_selection_device_manager_v1`: Creates primary selection sources and devices. Version 1.
- `zwp_primary_selection_device_v1`: Manages the primary selection for a seat.
- `zwp_primary_selection_offer_v1`: Offers data for the primary selection.
- `zwp_primary_selection_source_v1`: Provides data to the primary selection.

### Offer/Source Model

The compositor creates a device per seat. When a client sets a source, it becomes the primary selection. Other clients receive an offer when they request the selection data.

## zwp-relative-pointer

The `zwp_relative_pointer` module provides relative pointer motion events, useful for pointer-lock scenarios.

### Supported Interfaces

- `zwp_relative_pointer_manager_v1`: Creates relative pointer instances. Version 1.
- `zwp_relative_pointer_v1`: Receives relative motion events.

### Relative Motion Events

```rust
pub struct RelativeMotionEvent {
    pub time: u64,
    pub dx: f64,
    pub dy: f64,
    pub dx_unaccelerated: f64,
    pub dy_unaccelerated: f64,
}
```

### Pointer Lock

When a client locks the pointer, absolute motion events are replaced with relative motion events. The cursor is hidden and constrained to the client's surface.

### Confinement

The pointer can be confined to a specific surface. Movement outside the surface boundary is clamped.

## Protocol Version Negotiation

Waydri supports version negotiation for all protocols. The compositor advertises the highest version it supports, and the client connects at the negotiated version.

```rust
let max_version = protocol.version();
let negotiated = max_version.min(client_advertised_version);
```

## Event Dispatching

Incoming protocol events are dispatched through a handler registry:

```rust
protocol_handler.register::<XdgWmBase>(|event| {
    match event {
        XdgWmBaseEvent::GetXdgSurface { id } => { ... }
        XdgWmBaseEvent::Pong { serial } => { ... }
    }
});
```

Events are buffered and dispatched in order during the compositor's main loop iteration.
