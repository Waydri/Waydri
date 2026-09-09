# InputManager API

The `InputManager` handles all input device events and dispatches them to the appropriate handlers. It manages keyboard, pointer, and touch input with support for gesture recognition.

## Construction

```rust
let input = InputManager::new();
```

Creates a new InputManager with default configuration. All input devices are registered automatically when the Wayland session starts.

## Methods

### `dispatch(&mut self)`

Polls all registered input devices and dispatches pending events. Call this once per frame cycle from the compositor main loop. Returns immediately if no events are pending.

### `process_key_event(&mut self, event: KeyEvent) -> bool`

Processes a keyboard event including key press, release, and repeat. Returns `true` if the event was consumed by a binding or handler, `false` if it should be passed to the focused window.

```rust
let consumed = input.process_key_event(KeyEvent {
    key: KEY_A,
    state: KeyState::Pressed,
    modifiers: Modifiers::CTRL | Modifiers::ALT,
    time: timestamp,
});
```

### `process_pointer_event(&mut self, event: PointerEvent) -> bool`

Processes pointer motion, button press, button release, and scroll events. Returns `true` if the event was consumed. Handles pointer focus changes and cursor warping.

```rust
let consumed = input.process_pointer_event(PointerEvent::Motion {
    time: timestamp,
    delta_x: 5.0,
    delta_y: -3.0,
});
```

### `process_touch_event(&mut self, event: TouchEvent) -> bool`

Processes multi-touch events including touch begin, update, end, and cancel. Tracks up to 10 simultaneous touch points. Returns `true` if the event was consumed by a gesture or binding.

```rust
let consumed = input.process_touch_event(TouchEvent::Begin {
    time: timestamp,
    slot: 0,
    x: 500.0,
    y: 300.0,
});
```

## GestureRecognizer

The `GestureRecognizer` is embedded within `InputManager` and uses a state machine to detect multi-touch gestures.

### Gesture States

- `Idle`: No gesture detected, tracking raw touch points.
- `Tracking`: A gesture has started and is being tracked.
- `Recognized`: A complete gesture has been identified.
- `Failed`: The current gesture sequence was canceled.

### Supported Gestures

- **Tap**: Single or multi-finger tap with configurable timeout.
- **Pinch**: Two-finger pinch in/out with distance threshold.
- **Swipe**: Directional swipe with minimum distance and velocity.
- **Long Press**: Single finger held beyond time threshold.

### Configuration

```rust
let gesture_config = GestureConfig {
    tap_timeout_ms: 200,
    swipe_threshold: 50.0,
    pinch_threshold: 0.3,
    long_press_ms: 500,
};
```

## Event Types

### KeyEvent

```rust
pub struct KeyEvent {
    pub key: u32,
    pub state: KeyState,
    pub modifiers: Modifiers,
    pub time: u64,
}

pub enum KeyState {
    Pressed,
    Released,
    Repeated,
}
```

### PointerEvent

```rust
pub enum PointerEvent {
    Motion { time: u64, delta_x: f64, delta_y: f64 },
    Button { time: u64, button: u32, state: ButtonState },
    Axis { time: u64, axis: Axis, amount: f64 },
    Frame { time: u64 },
}
```

### TouchEvent

```rust
pub enum TouchEvent {
    Begin { time: u64, slot: u32, x: f64, y: f64 },
    Update { time: u64, slot: u32, x: f64, y: f64 },
    End { time: u64, slot: u32 },
    Cancel { time: u64 },
}
```

## Integration

The InputManager integrates with WindowManager for focus management, WorkspaceManager for workspace switching, and the IPC server for forwarding input events to clients.

```rust
let mut input = InputManager::new();
let mut window_mgr = WindowManager::new();

loop {
    input.dispatch();
    while let Some(event) = input.next_event() {
        match event {
            InputEvent::Key(ke) => {
                if !input.process_key_event(ke) {
                    window_mgr.send_key_to_focused(ke);
                }
            }
            InputEvent::Pointer(pe) => {
                input.process_pointer_event(pe);
            }
            InputEvent::Touch(te) => {
                input.process_touch_event(te);
            }
        }
    }
}
```
