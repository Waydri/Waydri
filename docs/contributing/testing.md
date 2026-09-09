# Test Structure

## Directory Layout

```
tests/
    unit/           Unit tests for individual modules
    integration/    Integration tests for component interaction
    protocols/      Wayland protocol compliance tests
    wayland/        Wayland session and display tests
    benchmarks/     Performance benchmarks
```

## Unit Tests

Located in `tests/unit/`. Test individual functions and structs in isolation.

```
tests/unit/
    config.rs
    layout.rs
    input.rs
    window.rs
    workspace.rs
    renderer.rs
    ipc.rs
    animation.rs
```

Unit tests verify correctness of individual components without depending on external systems.

## Integration Tests

Located in `tests/integration/`. Test interactions between multiple components.

```
tests/integration/
    compositor.rs
    plugin.rs
    multi_monitor.rs
    workspace_switch.rs
```

Integration tests simulate realistic scenarios involving multiple subsystems working together.

## Protocol Tests

Located in `tests/protocols/`. Verify Wayland protocol wire-format compliance.

```
tests/protocols/
    xdg_shell.rs
    layer_shell.rs
    primary_selection.rs
    relative_pointer.rs
```

Protocol tests verify that the compositor correctly encodes and decodes Wayland messages.

## Wayland Tests

Located in `tests/wayland/`. Test with actual Wayland displays and clients.

```
tests/wayland/
    display.rs
    client.rs
    surface.rs
```

Wayland tests require a running Wayland display and verify real protocol interactions.

## Benchmarks

Located in `tests/benchmarks/`. Performance benchmarks using `criterion`.

```
tests/benchmarks/
    render_bench.rs
    layout_bench.rs
    input_bench.rs
```

## Running Tests

### All Tests

```bash
cargo test --package waydri-core
```

### Integration Tests Only

```bash
cargo test --test integration_tests
```

### Unit Tests with Verbose Output

```bash
cargo test --test unit_tests -- --nocapture
```

### Tests by Module

```bash
cargo test --test unit_tests config
cargo test --test unit_tests layout
```

### Wayland Protocol Tests

```bash
cargo test --test wayland_tests
```

### Benchmarks

```bash
cargo bench --package waydri-core
```

## Test Conventions

- Test functions are named `test_<thing>_<scenario>`.
- Use `#[should_panic(expected = "...")]` for expected panics.
- Use `assert_eq!`, `assert_ne!`, `assert!` for assertions.
- Create test fixtures with builder functions, not global state.
- Clean up resources in test teardown (use `Drop` implementations).
- Tests must not depend on the order they run.
- Tests must not depend on external state (files, network, running services).
