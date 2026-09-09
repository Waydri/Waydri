# Contributing

Thank you for contributing to Waydri. Read this document before opening a
pull request.

## Code of conduct

All contributors must follow the Contributing Covenant in CODE_OF_CONDUCT.md.
Be respectful, constructive, and inclusive.

## Getting started

1. Fork the repository.
2. Create a branch with a descriptive name.
3. Make focused changes with clear commit messages.
4. Run the tests for the affected components.
5. Open a pull request against main.

## Commit messages

Use the imperative mood. Keep the subject under 72 characters. Reference
issues when relevant.

## Code style

- Rust code follows the formatting produced by cargo fmt.
- Zig code follows zig fmt.
- C code follows clang-format with the project style.
- Kotlin code follows the Android Kotlin conventions.
- Do not add comments that restate the code. Use comments only when they
  explain a non-obvious decision.

## Testing

- Unit tests cover pure logic: utils, animation, layout, config, effects.
- Integration tests cover the compositor, renderer, and input pipelines.
- Protocol tests cover the Wayland wire format helpers.
- Benchmarks measure performance-sensitive paths.

Run the Rust tests with cargo from the core directory.

    cargo test

## Adding a file

Every file must contain real logic. Empty scaffolds and placeholder files
are not accepted. If a module needs no implementation yet, remove it from
the module tree instead of leaving a stub.

## Review

Maintainers review each pull request. Address review comments promptly.
Running the full test suite before requesting review reduces iteration time.