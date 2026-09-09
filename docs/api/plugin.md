# Plugin API

The plugin system allows extending Waydri with custom functionality through a sandboxed runtime.

## Plugin Trait

All plugins must implement the `Plugin` trait:

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_start(&mut self, ctx: &mut PluginContext) -> Result<()>;
    fn on_stop(&mut self);
    fn on_tick(&mut self, dt: f64, ctx: &mut PluginContext);
    fn capabilities(&self) -> Vec<Capability>;
}
```

### Lifecycle Methods

#### `name(&self) -> &str`

Returns the plugin's display name. Used in logs and IPC responses.

#### `version(&self) -> &str`

Returns the plugin's semantic version string.

#### `on_start(&mut self, ctx: &mut PluginContext) -> Result<()>`

Called when the plugin is loaded and enabled. Use this to initialize resources, register hooks, and set up event listeners. Return an error to prevent the plugin from loading.

#### `on_stop(&mut self)`

Called when the plugin is unloaded or the compositor shuts down. Clean up all resources here.

#### `on_tick(&mut self, dt: f64, ctx: &mut PluginContext)`

Called once per compositor frame. `dt` is the time in seconds since the last tick. Use this for animations, polling, and periodic tasks.

#### `capabilities(&self) -> Vec<Capability>`

Returns the list of capabilities the plugin requires. The sandbox enforces these permissions at runtime.

## PluginManifest

Plugin metadata is defined in a `manifest.json` file:

```json
{
    "name": "my-plugin",
    "version": "1.0.0",
    "description": "A custom plugin",
    "author": "developer",
    "entry": "libmy_plugin.so",
    "capabilities": ["network", "rpc"],
    "min_waydri_version": "0.10.0",
    "sandbox": {
        "policy": "strict",
        "network_hosts": ["api.example.com"],
        "filesystem_read": ["/etc/my-plugin/"]
    }
}
```

### `PluginManifest::from_json(path: &Path) -> Result<PluginManifest>`

Parses a manifest file and validates all required fields. Returns an error with descriptive messages for invalid manifests.

## Capability Types

| Capability | Access |
|-----------|--------|
| `network` | Outbound network connections to configured hosts |
| `fs_read` | Read access to specified filesystem paths |
| `fs_write` | Write access to specified filesystem paths |
| `compositor` | Access to compositor state (windows, outputs) |
| `input` | Ability to generate synthetic input events |
| `rpc` | IPC server and client communication |
| `animation` | Access to animation and effects pipeline |

## PluginContext

The context object provided to plugin callbacks:

```rust
pub struct PluginContext {
    pub compositor: CompositorState,
    pub windows: WindowAccessor,
    pub outputs: OutputAccessor,
    pub config: ConfigAccessor,
}
```

### CompositorState

Provides read access to the current compositor state including focused window, active workspace, and output information.

### WindowAccessor

Allows plugins to query and modify window properties within their permitted scope.

### OutputAccessor

Provides information about connected outputs and their current modes.

### ConfigAccessor

Allows plugins to read configuration values from the Waydri config tree.

## SandboxPolicy

```rust
pub enum SandboxPolicy {
    Strict,
    Permissive,
    Disabled,
}
```

- **Strict**: Only explicitly declared capabilities are allowed. All access violations are denied and logged.
- **Permissive**: Capabilities beyond the manifest are allowed with a warning logged.
- **Disabled**: No sandbox enforcement. Use only for trusted first-party plugins.

## Plugin Registration

Plugins are registered in the Waydri config:

```lua
return {
    plugins = {
        {
            path = "/usr/lib/waydri/plugins/statusbar.so",
            enabled = true,
        },
        {
            path = "/usr/lib/waydri/plugins/notifier.so",
            enabled = true,
        },
    },
}
```

## Error Handling

Plugin errors are caught and logged without crashing the compositor. A plugin that panics in `on_tick` is forcibly unloaded. Repeated errors within a short window cause the plugin to be disabled until the compositor restarts.
