# Code Review Checklist

## Functionality

- [ ] All tests pass.
- [ ] No compiler warnings (`cargo clippy` is clean).
- [ ] No clippy warnings (`cargo clippy -- -D warnings`).
- [ ] Code contains real implementations, not stubs or placeholders.
- [ ] Edge cases are handled (empty inputs, boundary values, error paths).
- [ ] Performance characteristics are reasonable (no O(n^2) where O(n) suffices).

## Code Quality

- [ ] Code follows the project style guide.
- [ ] No unnecessary comments (code is self-documenting).
- [ ] No TODO/FIXME markers left in code.
- [ ] Function and type names are descriptive and consistent.
- [ ] No dead code or unused imports.
- [ ] Error types are properly defined and propagated.
- [ ] Public APIs have explicit type annotations.

## Security

- [ ] No secrets or keys in source code or config files.
- [ ] No hardcoded paths that could be exploited.
- [ ] Input validation is performed at API boundaries.
- [ ] Unsafe code blocks are justified and audited.
- [ ] File operations use appropriate permissions.
- [ ] Network operations handle malformed responses.

## Tests

- [ ] Unit tests cover new public functions.
- [ ] Integration tests verify end-to-end behavior.
- [ ] Edge cases are tested.
- [ ] Error paths are tested.
- [ ] Tests are independent and do not depend on execution order.
- [ ] No flaky tests (random failures).

## Documentation

- [ ] API documentation is updated for changed public interfaces.
- [ ] Configuration changes are documented.
- [ ] Breaking changes are noted in the changelog.
- [ ] README is updated if setup steps change.

## Compatibility

- [ ] Changes are backward-compatible unless intentional.
- [ ] Android builds are not broken.
- [ ] Linux builds are not broken.
- [ ] No new platform-specific code without conditional compilation.

## Merge Readiness

- [ ] All CI checks pass.
- [ ] No merge conflicts with the target branch.
- [ ] At least one maintainer has approved the PR.
- [ ] All review comments have been addressed.
- [ ] PR description accurately describes the changes.
