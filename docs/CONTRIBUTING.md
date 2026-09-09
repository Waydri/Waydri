# Contributing to Waydri

Thank you for contributing. This document outlines the contribution workflow for the Waydri project.

## Code Style

- Rust code is formatted with `rustfmt` (default settings).
- Run `cargo fmt -- --check` before committing.
- Clippy lints with `-D warnings` must pass: `cargo clippy -- -D warnings`.
- Shell scripts follow `set -euo pipefail` and are shellcheck-clean.
- C code in `xwayland/` follows the project's formatting and includes guards.

## Testing

- All new behavior must have tests.
- Run the full suite: `tools/test/run_all`.
- Unit tests live in `tests/unit`, integration tests in `tests/integration`, and Wayland protocol tests in `tests/wayland`.
- Benchmarks in `benches/` should be updated when affecting the compositor hot path.

## Pull Request Process

1. Fork the repository and create a feature branch.
2. Make your changes, keeping commits focused.
3. Run format, lint, and the full test suite.
4. Push your branch and open a pull request.
5. Reference any related issue in the PR description.
6. Maintainers review and may request changes; address feedback.

## Commit Format

Commits use a conventional format:

```
<type>(<scope>): <summary>

<body>
```

Types: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `chore`.

Scopes include: `core`, `compositor`, `renderer`, `render`, `layout`, `input`, `ipc`, `config`, `android`, `linux`, `xwayland`, `docs`, `tools`.

Example:

```
feat(compositor): add vblank-synced frame pacing
```

## Review

See [docs/contributing/review.md](contributing/review.md) for review guidelines and [docs/contributing/code_style.md](contributing/code_style.md) for the detailed style reference.
