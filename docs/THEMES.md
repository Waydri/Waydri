# Waydri Themes

Themes define the visual appearance of Waydri: colors, fonts, border styling, shadows, and effects. Themes are stored in `config/themes/*.json` and are hot-reloadable.

## Built-In Themes

| Theme | Palette |
|-------|---------|
| `waydri` | Default branded theme |
| `dark` | Dark surfaces, blue accent |
| `light` | Light surfaces |
| `dracula` | Dracula purple/green |
| `gruvbox` | Gruvbox warm |
| `nord` | Nord cold |
| `tokyonight` | Tokyo Night blue |
| `solarized` | Solarized aqua |
| `catppuccin` | Catppuccin Mocha |
| `monokai` | Monokai |
| `one-dark` | One Dark |
| `oceanic` | Oceanic |
| `github` | GitHub light |
| `everforest` | Everforest |
| `rosepine` | Rosé Pine |
| `spacegray` | Space Gray |
| `ayu` | Ayu |
| `base16` | Base16 |
| `hypr` | Hyprland-inspired |
| `zenburn` | Zenburn |

## Selecting a Theme

```toml
[theme]
name = "dark"
```

Apply at runtime:

```
waydri-ipc theme.set nord
```

## Color Format

Colors are hex values in `#RRGGBB` or `#RRGGBBAA`:

```json
{
  "colors": {
    "background": "#282a36",
    "accent": "#bd93f9"
  }
}
```

The following color roles are available: `background`, `foreground`, `primary`, `secondary`, `accent`, `urgent`, `inactive`, `border`, `shadow`.

## Custom Themes

Create a file `config/themes/mytheme.json`:

```json
{
  "name": "mytheme",
  "colors": {
    "background": "#1e1e2e",
    "foreground": "#cdd6f4",
    "primary": "#89b4fa",
    "secondary": "#313244",
    "accent": "#cba6f7",
    "urgent": "#f38ba8"
  },
  "font": {
    "family": "Fira Code",
    "size": 11
  },
  "decoration": {
    "border_width": 2,
    "border_radius": 8,
    "shadow": true,
    "shadow_blur": 18,
    "shadow_offset": [0, 2],
    "shadow_opacity": 0.4
  }
}
```

## Effects

```json
{
  "effects": {
    "opacity_focused": 1.0,
    "opacity_unfocused": 0.9,
    "blur": true,
    "blur_size": 8,
    "vibrance": 1.0
  }
}
```

## Applying Themes

- On startup from the `theme.name` config value.
- At runtime through `waydri-ipc theme.set <name>`.
- By reloading the config with `waydri-ipc reload`, which reapplies the configured theme.
