# IPC API

The IPC (Inter-Process Communication) system provides a Unix socket-based protocol for external tools and scripts to communicate with the Waydri compositor.

## IpcServer

The server runs inside the compositor process and handles incoming connections from clients.

### Construction

```rust
let server = IpcServer::new("/run/user/1000/waydri/ipc.sock")?;
```

Creates an IPC server bound to the specified Unix socket path. The socket file is created with mode 0600 for security.

### Methods

#### `start(&mut self) -> Result<()>`

Starts listening for incoming connections. Non-blocking; spawns internal connection handler.

#### `stop(&mut self)`

Stops the server, closes all client connections, and removes the socket file.

#### `send_event(&self, event: Event) -> Result<()>`

Broadcasts an event to all connected clients. Events are serialized as JSON and sent as newline-delimited messages.

```rust
server.send_event(Event::WindowCreated {
    window_id: 42,
    title: "Alacritty".to_string(),
    app_id: "org.alacritty".to_string(),
})?;
```

#### `handle_connection(&mut self) -> Result<()>`

Accepts pending connections and processes incoming commands. Call this from the compositor event loop. Returns an error only on fatal socket failure.

### Client Tracking

The server maintains a list of connected clients. Each client is identified by its peer credentials (UID, PID). Stale connections are automatically cleaned up on read failure.

## IpcClient

The client connects to the compositor from external processes.

### Construction

```rust
let client = IpcClient::connect("/run/user/1000/waydri/ipc.sock")?;
```

Connects to the Waydri IPC socket. Returns an error if the socket does not exist or the connection is refused.

### Methods

#### `send_command(&self, command: Command) -> Result<Event>`

Sends a command and waits for the compositor's response. Blocks until the response is received or the connection times out after 5 seconds.

```rust
let response = client.send_command(Command::GetVersion)?;
match response {
    Event::VersionInfo { version } => println!("Waydri {}", version),
    _ => {}
}
```

#### `send_command_async(&self, command: Command) -> Result<()>`

Sends a command without waiting for a response. Useful for fire-and-forget commands like `Quit`.

## Protocol

All messages are JSON-encoded and newline-delimited. The protocol is versioned and backward-compatible.

### Command Format

```json
{
    "type": "command",
    "id": "unique-request-id",
    "command": "set_layout",
    "args": {
        "layout": "dwindle"
    }
}
```

### Response Format

```json
{
    "type": "response",
    "id": "unique-request-id",
    "success": true,
    "data": {}
}
```

### Event Format

```json
{
    "type": "event",
    "event": "window_created",
    "data": {
        "window_id": 42,
        "title": "Alacritty",
        "app_id": "org.alacritty"
    },
    "timestamp": 1700000000000
}
```

## Supported Commands

| Command | Description | Args |
|---------|-------------|------|
| `quit` | Shut down the compositor | none |
| `reload_config` | Reload configuration files | none |
| `set_layout` | Set layout for current workspace | `layout: string` |
| `next_workspace` | Switch to next workspace | none |
| `prev_workspace` | Switch to previous workspace | none |
| `switch_workspace` | Switch to specific workspace | `index: number` |
| `focus_window` | Focus a window by ID | `window_id: number` |
| `close_window` | Close a window by ID | `window_id: number` |
| `set_opacity` | Set window opacity | `window_id: number, opacity: number` |
| `toggle_fullscreen` | Toggle fullscreen on focused window | none |
| `toggle_floating` | Toggle floating on focused window | none |
| `list_windows` | List all managed windows | none |
| `get_version` | Get compositor version | none |

## Supported Events

| Event | Description |
|-------|-------------|
| `window_created` | A new window was mapped |
| `window_destroyed` | A window was unmapped |
| `window_focus_changed` | Focus moved to a different window |
| `workspace_changed` | Active workspace changed |
| `layout_changed` | Layout changed on current workspace |
| `output_added` | A new output was connected |
| `output_removed` | An output was disconnected |
| `frame_completed` | A render frame completed |
| `damaged_region` | A damaged screen region was recorded |
