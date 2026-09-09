# Waydri Keybindings

Keybindings map key combinations to compositor actions. They are defined in the `keybinds` section of the configuration file.

## Default Keybindings

The default modifier is `mod`, which is `SUPER` (the Windows/Command key) on Linux and the system overlay key on Android.

| Key Combination | Action |
|-----------------|--------|
| `mod+Return` | Open terminal |
| `mod+d` | Open application launcher |
| `mod+q` | Close focused window |
| `mod+Shift+Space` | Toggle floating |
| `mod+Shift+f` | Toggle fullscreen |
| `mod+Left` | Focus previous window |
| `mod+Right` | Focus next window |
| `mod+Up` | Focus window above |
| `mod+Down` | Focus window below |
| `mod+Shift+Left` | Move window left |
| `mod+Shift+Right` | Move window right |
| `mod+Shift+Up` | Move window up |
| `mod+Shift+Down` | Move window down |
| `mod+1` .. `mod+9` | Switch to workspace 1..9 |
| `mod+Shift+1` .. `9` | Move window to workspace |
| `mod+space` | Cycle layouts |
| `mod+[` / `mod+]` | Cycle through windows |
| `mod+Escape` | Lock screen |
| `mod+Shift+e` | Exit compositor |
| `mod+volume_up` / `volume_down` | Adjust volume |

## Action Reference

| Action | Description |
|--------|-------------|
| `exec <cmd>` | Run a shell command |
| `close` | Close the focused window |
| `focus next` / `focus prev` | Move focus |
| `focus prev / next` | Cycle focus |
| `move next` / `move prev` | Move window in the tiling order |
| `toggle_float` | Toggle between floating and tiled |
| `toggle_fullscreen` | Toggle fullscreen |
| `workspace <n>` | Switch to workspace |
| `move_workspace <n>` | Move window to workspace |
| `cycle_layout` | Cycle the active layout |
| `layout <name>` | Set the layout |
| `resize_master` / `resize_slave` | Adjust master ratio |
| `spawn` | Launch configured autostart |

## Custom Keybindings

Add custom bindings by editing the `keybinds` table:

```toml
[keybinds]
"mod+c" = "exec code"
"mod+Shift+c" = "close"
"mod+Ctrl+Return" = "exec kitty --class terminal"
"XF86AudioPlay" = "exec playerctl play-pause"
```

Bindings can be added and removed at runtime:

```
waydri-ipc keybind.add "mod+c" "exec code"
waydri-ipc keybind.remove "mod+c"
```

## Key Format Syntax

A key combination is one or more modifiers joined by `+` followed by a key name.

Modifiers:

- `mod` - primary modifier (SUPER by default)
- `Shift`
- `Ctrl`
- `Alt`
- `Super`
- `Meta`

Key names follow the evdev/XKB key names: `a`..`z`, `0`..`9`, `Return`, `Space`, `Tab`, `Escape`, `BackSpace`, `Delete`, `Home`, `End`, `Page_Up`, `Page_Down`, `Left`, `Right`, `Up`, `Down`, `F1`..`F12`, and media keys such as `XF86AudioPlay`.

A bare key (no modifier) is written without `+`, for example `Print` = `mod` is required for compositor conflicts.
