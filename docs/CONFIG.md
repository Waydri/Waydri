# Waydri Configuration Reference

Waydri reads configuration from `waydri.toml`, `waydri.json`, or `waydri.lua`. The file format is detected by extension. TOML is recommended.

## Configuration File Locations

Resolved in order of priority:

1. `$WAYDRI_CONFIG` if set
2. `$XDG_CONFIG_HOME/waydri/waydri.toml` (default `~/.config/waydri/`)
3. `./waydri.toml` in the working directory
4. `/etc/xdg/waydri/config.toml`
5. Built-in defaults (`/usr/share/waydri/default_config.toml`)

## Top-Level Sections

| Section | Purpose |
|---------|---------|
| `general` | Compositor-wide settings |
| `keybinds` | Key-to-action mappings |
| `layout` | Default layout and ratios |
| `theme` | Visual theme selection |
| `animations` | Animation configuration |
| `rules` | Window matching rules |
| `plugins` | Plugin loading list |

## general

```toml
[general]
name = "waydri"
max_fps = 144
border_size = 2
gaps = 8
smart_gaps = false
corner_radius = 8
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `name` | string | `waydri` | Compositor instance name |
| `max_fps` | integer | 60 | Frame rate cap |
| `border_size` | integer | 2 | Window border width |
| `gaps` | integer | 8 | Inter-window gap |
| `smart_gaps` | boolean | false | Hide gaps for a single window |
| `corner_radius` | integer | 0 | Window corner radius |

## keybinds

```toml
[keybinds]
"mod+Return" = "exec alacritty"
"mod+q" = "close"
"mod+Shift+Space" = "toggle_float"
```

The left side is a key combination with modifiers separated by `+`. The right side is an action string. See [KEYBINDS.md](KEYBINDS.md) for the action reference.

## layout

```toml
[layout]
primary = "master_stack"
master_size = 1
master_ratio = 0.55
splits = true
```

See [LAYOUTS.md](LAYOUTS.md) for the layout reference.

## theme

```toml
[theme]
name = "dracula"
accent = "#bd93f9"
```

See [THEMES.md](THEMES.md).

## animations

```toml
[animations]
enabled = true
default_spring = "smooth"
easing = "ease_in_out_cubic"
duration = 150
```

See [ANIMATIONS.md](ANIMATIONS.md).

## rules

```toml
[[rules]]
match = { class = "^(Firefox)$" }
floating = false
```

Rules match windows and apply properties. See the rules section of [CONFIG guide](../tools/docs/config.md).

## plugins

```toml
[[plugins]]
name = "example"
path = "/opt/waydri/plugins/example.so"
enabled = true
```

See [PLUGINS.md](PLUGINS.md).

## TOML vs JSON vs Lua

The same structure is accepted in JSON with matching keys. Lua configuration runs in a sandbox and exposes a `config` table:

```lua
config.general.name = "waydri"
config.layout.primary = "grid"
```

All formats are hot-reloadable at runtime:

```
waydri-ipc reload
```
