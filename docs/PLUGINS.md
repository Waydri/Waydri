# Waydri Plugin System

Plugins extend Waydri with custom behavior using a dynamic library ABI. See `config/plugins/` for the plugin registry and `docs/examples/plugin_rust.rs` for a full Rust example.

## Writing Plugins

A plugin is a shared object exporting the Waydri plugin ABI. See [tools/docs/plugins.md](../tools/docs/plugins.md) for the complete ABI reference.

Example C plugin:

```c
#include "waydri/plugin.h"

int waydri_plugin_init(struct PluginApi* api) {
    api->log(api->context, 0, "hello from plugin");
    return 0;
}

void waydri_plugin_deinit(void) {
}
```

## API Hooks

Plugins can register lifecycle hooks:

- `window_focused` - called when focus changes
- `window_closed` - called when a window closes
- `window_created` - called when a window is created
- `frame_ready` - called after each composited frame
- `workspace_switched` - called on workspace change

## Capabilities

Capabilities gate sensitive operations and are requested at load time:

| Capability | Description |
|------------|-------------|
| `exec` | Spawn processes |
| `input` | Inject input events |
| `state` | Modify compositor state |
| `ipc` | Send IPC messages |
| `fs` | Broad filesystem access |
| `net` | Network access |

Declare required capabilities in the plugin config:

```toml
[[plugins]]
name = "myplugin"
path = "/opt/waydri/plugins/myplugin.so"
capabilities = ["exec", "ipc"]
enabled = true
```

A plugin that requests an ungranted capability fails to load with a descriptive error.

## Loading and Unloading

Load a plugin at runtime:

```
waydri-ipc plugin.load /opt/waydri/plugins/myplugin.so
```

Unload by name:

```
waydri-ipc plugin.unload myplugin
```

List loaded plugins:

```
waydri-ipc plugin.list
```

## Example Plugin (Rust)

See [docs/examples/plugin_rust.rs](examples/plugin_rust.rs) for a Rust plugin implementing a keybind and a focus hook via the C ABI.

## Sandbox Rules

- Plugins run in-process; a segfaulting plugin is unloaded on the next hook.
- Sandboxing via seccomp can be enabled at build time to restrict syscalls.
- Network access is disabled unless the `net` capability is granted.
- Plugins cannot read the keystore or modify other plugins' state.

## Plugin Registry

The plugin registry is stored in `config/plugins/enabled.json` and `config/plugins/available.json`. The enable list is authoritative for startup loading.
