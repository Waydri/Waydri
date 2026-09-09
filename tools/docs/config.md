# Waydri Configuration Guide

Waydri reads its configuration from `waydri.toml`, `waydri.json`, or `waydri.lua` files. Configuration is resolved by searching, in order:

1. `$XDG_CONFIG_HOME/waydri/` (default `~/.config/waydri/`)
2. `$WAYDRI_CONFIG` if set
3. `./waydri.toml` in the working directory
4. The built-in defaults shipped with the binary

The format is detected from the file extension. TOML is the recommended format.

## General Settings

```toml
[general]
name = "waydri"
max_fps = 144
border_size = 2
gaps = 8
smart_gaps = false
corner_radius = 8
resize_by_tiling = true
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `name` | string | `waydri` | Compositor instance name |
| `max_fps` | integer | 60 | Maximum frames per second |
| `border_size` | integer | 2 | Window border width in pixels |
| `gaps` | integer | 8 | Gap between windows in pixels |
| `smart_gaps` | boolean | false | Hide gaps when only one window |
| `corner_radius` | integer | 0 | Rounded corner radius |
| `resize_by_tiling` | boolean | true | Resize tiled neighbors on split |

## Keybinds

```toml
[keybinds]
"mod+Return" = "exec alacritty"
"mod+d" = "exec wofi --show drun"
"mod+Shift+f" = "fullscreen"
"mod+q" = "close"
"mod+Shift+Space" = "toggle_float"
"mod+Left" = "focus prev"
"mod+Right" = "focus next"
"mod+1" .. "mod+9" = "workspace 1"
```

Keybinds map a key combination to an action. Modifiers are separated by `+`. See the keybind format reference in the main documentation.

## Layout

```toml
[layout]
primary = "master_stack"
master_size = 1
master_ratio = 0.55
splits = true
```

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `primary` | string | `master_stack` | Default layout type |
| `master_size` | integer | 1 | Number of master windows |
| `master_ratio` | float | 0.55 | Master area ratio |
| `splits` | boolean | true | Enable manual splits |

## Theme

```toml
[theme]
name = "dracula"
accent = "#bd93f9"
background = "#282a36"
foreground = "#f8f8f2"
border = "#6272a4"
font = "JetBrains Mono"
font_size = 12
animations = true
```

## Rules

Rules match windows by class, title, or app_id and apply properties.

```toml
[[rules]]
match = { class = "^(Firefox|Chromium)$" }
floating = true
border = "#ff5555"

[[rules]]
match = { title = ".*Password.*" }
opacity = 0.9
no_focus = false
```

## Plugins

```toml
[[plugins]]
name = "my-plugin"
path = "/opt/waydri/plugins/my-plugin.so"
enabled = true
config = { option = "value" }
```

## JSON Format

The same structure in JSON:

```json
{
  "general": { "name": "waydri", "max_fps": 144 },
  "layout": { "primary": "grid" },
  "theme": { "accent": "#8be9fd" }
}
```

## Lua Format

```lua
general.name = "waydri"
general.max_fps = 144
layout.primary = "dwindle"
theme.accent = "#ff6e6e"
```

Lua configuration runs in a sandboxed Lua interpreter and has access to the `config` table and the `waydri` global for defining rules and plugins inline.
