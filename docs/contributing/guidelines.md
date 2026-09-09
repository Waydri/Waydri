# Contributing Guidelines

## How to Contribute

### Fork and Clone

1. Fork the repository on GitHub.
2. Clone your fork: `git clone https://github.com/your-username/Waydri.git`
3. Add the upstream remote: `git remote add upstream https://github.com/Waydri/Waydri.git`
4. Create a feature branch: `git checkout -b feature/my-change`

### Branch Naming

- `feature/description`: New features.
- `fix/description`: Bug fixes.
- `docs/description`: Documentation changes.
- `refactor/description`: Code refactoring without behavior changes.
- `test/description`: Adding or updating tests.

### Commit Messages

Write commit messages in imperative mood. Start with a verb that describes what the commit does.

```
Add touch gesture recognition support

Implement pinch, swipe, and tap gesture detection in InputManager.
Supports up to 10 simultaneous touch points with configurable thresholds.
```

- First line: imperative summary, 72 characters max.
- Blank line separator.
- Body: explain what and why, not how. Wrap at 80 characters.

### Pull Request Format

1. Title: imperative summary matching commit style.
2. Description: what the PR does, why it is needed, and how to test it.
3. Reference related issues with `Fixes #123` or `Relates to #456`.
4. Ensure all CI checks pass before requesting review.

### Code Requirements

- No placeholder implementations. Every function must contain real logic.
- No comments in code unless specifically requested.
- No emoji in source files or commit messages.
- All public APIs must have type annotations.
- Follow the existing code style in the file you are editing.
- Add tests for new functionality.

### Testing

Run the full test suite before submitting:

```bash
cargo test --package waydri-core
cargo test --test integration_tests
cargo test --test wayland_tests
cargo bench --package waydri-core
```

Ensure no warnings with:

```bash
cargo clippy --package waydri-core -- -D warnings
```

### Review Process

1. All PRs require at least one review from a maintainer.
2. Reviewers check for correctness, style, tests, and documentation.
3. Address all review comments before merging.
4. Squash and merge is the default merge strategy.

### Reporting Issues

- Use the GitHub issue tracker.
- Include steps to reproduce.
- Include your system information (OS, GPU, driver version).
- Include log output with `RUST_LOG=trace`.
- Check existing issues before creating a new one.

### Code of Conduct

Follow the project Code of Conduct. Be respectful and constructive in all interactions.
