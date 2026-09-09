# Waydri Plugin Development

Plugins extend Waydri's functionality through a dynamic library ABI. Plugins are loaded from `~/.config/waydri/plugins/` or from paths specified in the config file.

## Plugin ABI

Plugins are shared objects that export a C-compatible ABI. The core functions are:

| Symbol | Signature | Description |
|--------|-----------|-------------|
| `waydri_plugin_version` | `u32` | Return the ABI version (must be 1) |
| `waydri_plugin_name` | `const char*` | Return the plugin name |
| `waydri_plugin_init` | `int (PluginApi*)` | Initialize the plugin, returns 0 on success |
| `waydri_plugin_deinit` | `void` | Called on plugin unload |

## PluginApi

The `PluginApi` struct provides callbacks and access to compositor state.

```c
struct PluginApi {
    void* context;
    void (*log)(void* ctx, int level, const char* msg);
    int (*keybind_add)(void* ctx, const char* combo, const char* action);
    int (*keybind_remove)(void* ctx, const char* combo);
    void (*window_focused)(void* ctx, void* cb);
    void (*window_closed)(void* ctx, void* cb);
    void* (*get_state)(void* ctx);
};
```

## Lifecycle Hooks

Plugins are loaded after compositor init and before the main loop. They are unloaded during shutdown. Loading and unloading are triggered by config changes, the IPC `plugin.load` and `plugin.unload` commands, or the `waydri` CLI.

```toml
[[plugins]]
name = "my-plugin"
path = "/opt/waydri/plugins/my-plugin.so"
enabled = true
```

## Capability System

Before a plugin can access sensitive functionality it must declare capabilities. Capabilities are requested at load time and granted based on the sandbox policy.

| Capability | Description |
|------------|-------------|
| `exec` | Spawn external processes |
| `input` | Inject synthetic input events |
| `state` | Read or modify compositor state |
| `ipc` | Send and receive IPC messages |
| `fs` | Read files outside the config root |

If a plugin requests a denied capability it fails to load with an error message.

## Sandbox Rules

- Plugins run in the compositor process but may be sandboxed via `seccomp` if the sandbox feature is enabled at build time.
- File access outside plugin state is limited by default; `fs` capability is required for broad access.
- Network access requires the `net` capability, disabled by default.
- A plugin that crashes is isolated; the compositor detects the crash on next hook invocation and unloads the plugin.

## Example Plugin

A minimal plugin that logs focus changes:

```c
#include <stdio.h>
#include "waydri/plugin.h"

static struct PluginApi api;

static void on_focus(void* ctx, const void* window) {
    api.log(api.context, 0, "focus changed");
}

uint32_t waydri_plugin_version(void) {
    return WAYDRI_ABI_VERSION;
}

const char* waydri_plugin_name(void) {
    return "focus-logger";
}

int waydri_plugin_init(struct PluginApi* a) {
    api = *a;
    api.window_focused(api.context, on_focus);
    return 0;
}

void waydri_plugin_deinit(void) {
    api.log(api.context, 0, "plugin unloaded");
}
```

Build it with:

```
gcc -shared -fPIC -I core/include focus_logger.c -o focus-logger.so
```

## Loading and Unloading

From the config:

```toml
[[plugins]]
name = "focus-logger"
path = "./focus-logger.so"
enabled = true
```

Reload the config to load newly added plugins:

```
waydri-ipc reload
```

Load or unload at runtime:

```
waydri-ipc plugin.load /opt/waydri/plugins/focus-logger.so
waydri-ipc plugin.unload focus-logger
```
