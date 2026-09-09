# Getting Started

## Installation

### Linux

Build from source:

```bash
git clone https://github.com/Waydri/Waydri.git
cd Waydri
cargo build --release
```

Install the binary:

```bash
sudo cp target/release/waydri /usr/local/bin/
```

### Android

Build the APK:

```bash
cd android
./gradlew assembleDebug
adb install app/build/outputs/apk/debug/app-debug.apk
```

## First Run

### Linux

1. Ensure you have the required dependencies: wayland, vulkan-loader, mesa (for GLES).
2. Run Waydri from a TTY or from an existing compositor:

```bash
waydri
```

Or nested inside another compositor:

```bash
WAYLAND_DISPLAY=waydri-0 waydri
```

3. Set the display for clients:

```bash
export WAYLAND_DISPLAY=waydri-0
```

### Android

1. Open the Waydri app from your launcher.
2. Grant the requested permissions.
3. The compositor starts with the default configuration.

## Basic Configuration

Waydri loads configuration from `~/.config/waydri/config.lua`. Create this file:

```lua
return {
    general = {
        name = "my-setup",
        max_fps = 60,
        gaps = 4,
    },
    animations = {
        enabled = true,
        duration = 300,
    },
    effects = {
        shadows_enabled = true,
        rounded_corners = true,
        corner_radius = 8,
    },
}
```

## Keybinds

Default keybinds use Super as the modifier:

| Keybind | Action |
|---------|--------|
| Super+Return | Open terminal |
| Super+Q | Close window |
| Super+F | Toggle fullscreen |
| Super+Space | Cycle layout |
| Super+1-0 | Switch to workspace 1-10 |
| Super+Shift+1-0 | Move window to workspace |
| Super+J/K | Focus next/prev |
| Super+H/L | Resize master |
| Super+Shift+Q | Quit Waydri |

Custom keybinds in `config/keybinds/default.lua`:

```lua
return {
    { "Super+Return", "spawn", "alacritty" },
    { "Super+Q", "close_focused" },
    { "Super+F", "toggle_fullscreen" },
    { "Super+Space", "cycle_layout" },
    { "Super+1", "switch_workspace", 1 },
}
```

## Layouts

Waydri includes several built-in layouts:

- **tile**: Master-stack (default).
- **monocle**: Fullscreen stacking.
- **dwindle**: Binary split.
- **grid**: Grid arrangement.
- **center_master**: Centered master with stacks.

Cycle layouts with Super+Space or set a specific layout via IPC.

## Themes

Themes are JSON files in `config/themes/`. Switch themes by changing the theme path in config:

```lua
return {
    theme = "waydri",
}
```

Available themes: `default`, `dark`, `light`, `dracula`, `nord`, `catppuccin`, `gruvbox`, `ayu`, `tokyonight`, `waydri`, `zenburn`.
