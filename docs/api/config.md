# Config API

The `waydri_core::config` module handles configuration loading, validation, and access.

## Types

### `Config`

The parsed configuration root.

```rust
pub struct Config {
    pub general: GeneralConfig,
    pub keybinds: KeybindConfig,
    pub layout: LayoutConfig,
    pub theme: ThemeConfig,
    pub animations: AnimationConfig,
    pub rules: Vec<Rule>,
    pub plugins: Vec<PluginConfig>,
}
```

Methods:

| Method | Signature | Description |
|--------|-----------|-------------|
| `load` | `fn load(path: &Path) -> Result<Self>` | Load and parse a config file |
| `load_default` | `fn load_default() -> Result<Self>` | Load the packaged default config |
| `resolve` | `fn resolve() -> Result<Self>` | Resolve config from search paths |
| `reload` | `fn reload(&mut self, path: &Path) -> Result<()>` | Reload from disk |
| `validate` | `fn validate(&self) -> Result<()>` | Validate values |

### `GeneralConfig`

```rust
pub struct GeneralConfig {
    pub name: String,
    pub max_fps: u32,
    pub border_size: u32,
    pub gaps: u32,
    pub smart_gaps: bool,
    pub corner_radius: u32,
}
```

## Module Support

Waydri supports TOML, JSON, and Lua configuration. The module exposes a unified `Config` regardless of source format.

```rust
use waydri_core::config::Config;

let config = Config::resolve()?;
println!("max fps: {}", config.general.max_fps);
```

## Hot Reload

Call `reload` to re-read config at runtime. The compositor applies the new values:

```rust
config.reload(&path)?;
```

Errors during reload are returned and the previous configuration is preserved.

## Example

```rust
use waydri_core::config::Config;

fn reload_loop() -> Result<()> {
    let mut config = Config::resolve()?;
    let path = config.source_path();
    loop {
        // wait for change...
        config.reload(&path)?;
    }
}
```
