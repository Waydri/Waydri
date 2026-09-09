# Code Style Guide

## Rust

### Indentation

Use 4 spaces for indentation. No tabs.

### Naming

- Types: `PascalCase`
- Functions and methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`
- Lifetimes: `'a`, `'b`, `'ctx`

### Formatting

- Open braces on the same line as the statement (K&R style).
- Close braces on their own line.
- Single expressions in `if`/`match`/`loop` blocks may stay on one line.
- Maximum line length: 100 characters.

```rust
pub fn process_event(event: &Event, ctx: &mut Context) -> Result<()> {
    match event {
        Event::Key(key) => ctx.handle_key(key),
        Event::Pointer(ptr) => ctx.handle_pointer(ptr),
        _ => Ok(()),
    }
}
```

### Derive Macros

Prefer derive macros over manual implementations when available:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
```

### Type Annotations

Use explicit types on public APIs and complex expressions. Inferred types are acceptable for local variables with clear constructors.

```rust
pub fn windows(&self) -> &[Window] {
    &self.window_list
}

let count: usize = windows.iter().filter(|w| w.visible).count();
```

### Error Handling

Use `thiserror` for library error types. Use `anyhow` for application-level error handling.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("file not found: {0}")]
    FileNotFound(PathBuf),
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}
```

## Zig

### Indentation

Use 4 spaces for indentation. No tabs.

### Naming

- Types: `PascalCase`
- Functions: `camelCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Variables: `snake_case`

### Style

- Prefer `const` over `var` when possible.
- Use `errdefer` for cleanup on error paths.
- Prefer explicit error handling over `catch` with `null`.

## Lua

### Indentation

Use 4 spaces for indentation. No tabs.

### Naming

- Functions and variables: `snake_case`
- Constants: `UPPER_SNAKE_CASE`
- Modules: `snake_case`

### Style

- Use `local` for all variable declarations.
- Prefer `if ... then ... end` over inline conditionals.
- Tables use trailing comma for the last element.

```lua
local config = {
    name = "default",
    max_fps = 60,
    gaps = 4,
}

local function create_window(props)
    return {
        title = props.title or "Untitled",
        width = props.width or 800,
    }
end
```

## C

### Indentation

Use 4 spaces for indentation. No tabs.

### Naming

- Types: `snake_case` with `_t` suffix for typedefs.
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE` macros.
- Variables: `snake_case`

### Style

- K&R brace style: opening brace on the same line.
- Declare variables at the top of the block.
- Use `static` for file-scoped variables.
- Prefer `size_t` for sizes and indices.
