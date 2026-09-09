# Writing Plugins

## Plugin API Overview

Plugins extend Waydri with custom functionality. They are loaded as shared libraries and run inside the compositor process under a sandbox.

## Manifest

Every plugin requires a `manifest.json`:

```json
{
    "name": "my-plugin",
    "version": "1.0.0",
    "description": "Does something useful",
    "author": "your-name",
    "entry": "libmy_plugin.so",
    "min_waydri_version": "0.11.0",
    "capabilities": ["compositor", "rpc"],
    "sandbox": {
        "policy": "strict"
    }
}
```

### Required Fields

- `name`: Unique plugin identifier.
- `version`: Semantic version.
- `entry`: Shared library filename relative to the plugin directory.

### Optional Fields

- `description`: Human-readable description.
- `author`: Plugin author name.
- `min_waydri_version`: Minimum Waydri version required.
- `capabilities`: List of required capabilities.
- `sandbox`: Sandbox configuration.

## Capabilities

Declare what your plugin needs access to:

| Capability | Purpose |
|-----------|---------|
| `compositor` | Read window state, output info, workspace data |
| `rpc` | Send/receive IPC messages |
| `network` | Outbound network connections |
| `fs_read` | Read files from specified paths |
| `fs_write` | Write files to specified paths |
| `input` | Generate synthetic input events |
| `animation` | Hook into animation pipeline |

## Lifecycle Hooks

### `on_start`

Called when the plugin is loaded. Initialize resources here:

```rust
fn on_start(&mut self, ctx: &mut PluginContext) -> Result<()> {
    let count = ctx.windows.count();
    log::info!("Plugin started, {} windows visible", count);
    Ok(())
}
```

### `on_stop`

Called when the plugin is unloaded. Clean up resources:

```rust
fn on_stop(&mut self) {
    self.connections.clear();
    log::info!("Plugin stopped");
}
```

### `on_tick`

Called once per frame. `dt` is seconds since the last tick:

```rust
fn on_tick(&mut self, dt: f64, ctx: &mut PluginContext) {
    self.timer += dt;
    if self.timer >= self.update_interval {
        self.timer = 0.0;
        self.update(ctx);
    }
}
```

## Plugin Context

The `PluginContext` provides access to compositor state:

```rust
pub struct PluginContext {
    pub compositor: CompositorState,
    pub windows: WindowAccessor,
    pub outputs: OutputAccessor,
    pub config: ConfigAccessor,
}
```

### WindowAccessor

```rust
let count = ctx.windows.count();
let focused = ctx.windows.focused_id();
let windows: Vec<WindowInfo> = ctx.windows.list();
```

### OutputAccessor

```rust
let output_count = ctx.outputs.count();
let primary = ctx.outputs.primary();
let resolution = primary.size();
```

### ConfigAccessor

```rust
let value = ctx.config.get("general.max_fps");
let theme = ctx.config.get_string("theme");
```

## Installation

Place your plugin in the Waydri plugin directory:

```bash
cp libmy_plugin.so ~/.config/waydri/plugins/
cp manifest.json ~/.config/waydri/plugins/
```

Enable in config:

```lua
return {
    plugins = {
        {
            path = "~/.config/waydri/plugins/my_plugin.so",
            enabled = true,
        },
    },
}
```

## Error Handling

- Plugin errors in `on_start` prevent the plugin from loading.
- Plugin errors in `on_tick` are logged but the plugin continues running.
- A plugin that panics is forcibly unloaded.
- Repeated crashes within 60 seconds cause the plugin to be disabled.

## Debugging Plugins

Run Waydri with plugin logging enabled:

```bash
RUST_LOG=waydri_core::plugin=trace waydri
```

Check the plugin sandbox logs for denied operations:

```bash
RUST_LOG=waydri_core::sandbox=debug waydri
```
