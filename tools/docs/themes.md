# Waydri Theme System

Waydri themes control the visual appearance of window decorations, borders, backgrounds, and effects. Themes are defined in TOML, JSON, or Lua and applied at runtime.

## Theme File Location

Themes are searched in:

1. `~/.config/waydri/themes/<name>.toml`
2. `/usr/share/waydri/themes/<name>.toml`
3. Bundled themes embedded in the binary

Select a theme in the config:

```toml
[theme]
name = "dracula"
```

## Color Scheme

A theme defines a set of named colors. Colors are hex values in `#RRGGBB` or `#RRGGBBAA` format.

```toml
[colors]
background = "#282a36"
foreground = "#f8f8f2"
primary = "#6272a4"
secondary = "#44475a"
accent = "#bd93f9"
urgent = "#ff5555"
inactive = "#44475a"
```

| Key | Role |
|-----|------|
| `background` | Root surface background |
| `foreground` | Text and icon color |
| `primary` | Active window border |
| `secondary` | Inactive window border |
| `accent` | Focus ring and highlights |
| `urgent` | Urgent window indication |
| `inactive` | Dimmed elements |

## Font Configuration

```toml
[font]
family = "JetBrains Mono"
size = 12
weight = "normal"
style = "normal"
```

## Border and Shadow Styling

```toml
[decoration]
border_width = 2
border_radius = 8
shadow = true
shadow_blur = 16
shadow_offset = [0, 2]
shadow_opacity = 0.4
```

## Applying Themes

Themes can be applied at runtime via the IPC interface:

```
waydri-ipc theme.set dracula
waydri-ipc theme.list
```

Or by editing the config and reloading:

```
waydri-ipc reload
```

## Built-In Themes

| Name | Description |
|------|-------------|
| `default` | Neutral gray with blue accent |
| `dark` | Dark surfaces with teal accent |
| `light` | Light surfaces with dark text |
| `dracula` | Dracula palette |
| `gruvbox` | Gruvbox warm palette |
| `nord` | Nord cool palette |
| `tokyonight` | Tokyo Night palette |
| `solarized` | Solarized palette |
| `catppuccin` | Catppuccin Mocha palette |

## Custom Theme Example

```toml
name = "my-theme"

[colors]
background = "#1e1e2e"
foreground = "#cdd6f4"
primary = "#89b4fa"
secondary = "#313244"
accent = "#cba6f7"
urgent = "#f38ba8"

[font]
family = "Fira Code"
size = 11

[decoration]
border_width = 3
border_radius = 10
shadow = true
shadow_blur = 24
```
